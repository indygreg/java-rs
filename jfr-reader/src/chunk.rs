// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR Chunk Reading
//!
//! JFR recordings are logically streams of self-contained data structures called
//! "chunks."
//!
//! Each chunk consists of a 68 byte header represented by [ChunkHeader].
//! As chunks are written to disk, the header can be dynamically updated.
//! State in the header indicates if the chunk is still actively writing and
//! whether it is finished. Chunks also self-identify whether they are the
//! final chunk in a stream of chunks.
//!
//! Within each chunk are discrete events, modeled by [EventRecord]. There
//! are special events denoting metadata and constant pools.

use crate::{
    constant_pool::ConstantPoolEvent,
    error::{ParseResult, Result},
    event::EventRecord,
    metadata::{Metadata, MetadataHeader},
    resolver::EventResolver,
};
use nom::{
    bytes::streaming::{tag, take},
    error::context,
    number::streaming::{be_u16, be_u32, be_u64},
};

pub const MAGIC: [u8; 4] = *b"FLR\0";

/// Represents the header of a chunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkHeader {
    /// Major version.
    ///
    /// Versions 1 and 2 are defined as of Java 17.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Chunk size in bytes.
    pub chunk_size: u64,

    /// Location/offset of the constant pool relative to beginning of this header.
    pub constant_pool_position: u64,

    /// Location/offset of the metadata pool relative to beginning of this header.
    pub metadata_position: u64,

    /// Number of nanoseconds since UNIX epoch.
    pub nanoseconds_since_epoch: u64,

    /// Duration in nanoseconds.
    pub duration_nanoseconds: u64,

    /// Start time in ticks.
    pub start_ticks: u64,

    /// Number of ticks per second.
    pub ticks_per_second: u64,

    /// Additional state and flags.
    ///
    /// First byte is file state. Final byte is flags.
    ///
    /// If first byte is 0, the chunk is finished.
    /// If first byte is 255, the chunk isn't consistent yet. Readers should
    /// probably stop and poll later.
    ///
    pub state_and_flags: u32,
}

impl ChunkHeader {
    /// Length in bytes of the chunk header.
    ///
    /// Data after this offset will be events data.
    pub const HEADER_SIZE: u64 = 68;

    /// Parse a chunk header from an input slice.
    ///
    /// Input must be at least [Self::HEADER_SIZE] in length.
    pub fn parse(s: &[u8]) -> ParseResult<Self> {
        let (s, _) = tag(MAGIC)(s)?;
        let (s, major) = be_u16(s)?;
        let (s, minor) = be_u16(s)?;
        let (s, chunk_size) = be_u64(s)?;
        let (s, constant_pool_position) = be_u64(s)?;
        let (s, metadata_position) = be_u64(s)?;
        let (s, nanoseconds_since_epoch) = be_u64(s)?;
        let (s, duration_nanoseconds) = be_u64(s)?;
        let (s, start_ticks) = be_u64(s)?;
        let (s, ticks_per_second) = be_u64(s)?;
        let (s, state_and_flags) = be_u32(s)?;

        Ok((
            s,
            Self {
                major,
                minor,
                chunk_size,
                constant_pool_position,
                metadata_position,
                nanoseconds_since_epoch,
                duration_nanoseconds,
                start_ticks,
                ticks_per_second,
                state_and_flags,
            },
        ))
    }
}

/// Describes common properties of entities that can read JFR chunks.
pub trait ChunkReader<'a, 'reader: 'a> {
    /// Obtains the parsed header for this chunk.
    fn header(&'reader self) -> &'reader ChunkHeader;

    /// Obtain the parsed metadata header for this chunk.
    fn metadata_header(&'reader self) -> &'reader MetadataHeader;

    /// Resolves metadata in this chunk.
    fn metadata(&'reader self) -> Result<Metadata<'a>>;

    /// Iterate event records in this chunk.
    ///
    /// Iteration is performed in chunk order, first to last, until end of chunk is reached.
    ///
    /// The event stream may include constant pool and metadata events.
    ///
    /// The emitted event records are very lightly parsed. They only decode the
    /// size + type header and hold a reference to the event's full data. This
    /// allows callers to quickly filter on event types without having to pay a
    /// cost to fully decode each event.
    fn iter_event_records(
        &'reader self,
    ) -> Box<dyn Iterator<Item = Result<EventRecord<'a>>> + 'reader>;

    /// Resolve lightly parsed constant pool events in this chunk.
    ///
    /// The first constant pool is the one defined in the metadata header.
    /// We likely traverse the events in reverse input buffer order. But this
    /// is not strictly guaranteed.
    ///
    /// One could obtain constant pool events by iterating through all events
    /// and filtering for just the constant pool ones. This function should be
    /// more efficient since it follows the event offset chain within constant
    /// pool events and doesn't require decoding event headers for every event.
    fn iter_constant_pool_events(
        &'reader self,
    ) -> Box<dyn Iterator<Item = Result<ConstantPoolEvent<'a>>> + 'reader>;

    /// Obtain a resolver for this chunk.
    ///
    /// The resolver is used to perform the heavy lifting to assemble events
    /// using class/type metadata and constants pools.
    fn resolver(&'reader self) -> Result<EventResolver<'a>> {
        let metadata = self.metadata()?;

        let constant_pools = self
            .iter_constant_pool_events()
            .collect::<Result<Vec<_>>>()?;

        EventResolver::new(metadata, constant_pools.into_iter())
    }
}

/// Read chunks from memory slices.
///
/// Instances can only be constructed if the full chunk data is available.
pub struct SliceReader<'a> {
    /// Original source data for this chunk, including the header.
    data: &'a [u8],

    header: ChunkHeader,

    /// The parsed metadata header.
    metadata_header: MetadataHeader,

    /// Slice holding the full metadata event data.
    metadata_event_data: &'a [u8],
}

impl<'a> SliceReader<'a> {
    /// Resolve a full chunk backed by in-memory data.
    ///
    /// Returns the remaining bytes from the input slice after this chunk's
    /// reported size.
    ///
    /// Metadata is very lightly parsed: we attempt to parse the metadata header
    /// and resolve the advertised slice holding the metadata. But we don't parse
    /// the metadata records or attempt to resolve the data structures within.
    /// This behavior facilitates scanning chunks without incurring metadata
    /// loading overhead.
    pub fn new(data: &'a [u8]) -> Result<(&'a [u8], Self)> {
        let (_, header) = context("parsing chunk header", ChunkHeader::parse)(data)?;

        // chunk_data is inclusive of the header.
        let (remaining, chunk_data) =
            context("resolving all chunk data", take(header.chunk_size))(data)?;

        // The metadata is potentially all data from the advertised offset to the end of the chunk.
        let (metadata_raw, _) = context(
            "resolving potential metadata data",
            take(header.metadata_position),
        )(chunk_data)?;

        // Read the metadata header so we can truncate to the event data.
        let (_, metadata_header) =
            context("parsing metadata header", MetadataHeader::parse)(metadata_raw)?;

        let (_, metadata_event_data) = context(
            "resolving full metadata event data",
            take(metadata_header.size as usize),
        )(metadata_raw)?;

        Ok((
            remaining,
            Self {
                data: chunk_data,
                header,
                metadata_header,
                metadata_event_data,
            },
        ))
    }

    /// The total length of the chunk in bytes.
    pub fn chunk_size(&self) -> usize {
        self.data.len()
    }

    /// Attempt to parse a constant pool event at a given chunk offset.
    fn parse_constant_pool_event(&self, offset: usize) -> ParseResult<ConstantPoolEvent<'a>> {
        let (event_data, _) =
            context("resolving content pool event data", take(offset))(self.data)?;

        let (remaining, event) =
            context("resolving constant pool event", ConstantPoolEvent::parse)(event_data)?;

        Ok((remaining, event))
    }
}

impl<'a, 'reader: 'a> ChunkReader<'a, 'reader> for SliceReader<'a> {
    fn header(&'reader self) -> &ChunkHeader {
        &self.header
    }

    fn metadata_header(&'reader self) -> &MetadataHeader {
        &self.metadata_header
    }

    fn metadata(&'reader self) -> Result<Metadata<'a>> {
        // This redundantly parses the header. But that should be trivial overhead.
        let (_, metadata) = Metadata::parse(self.metadata_event_data)?;

        Ok(metadata)
    }

    /// Iterate all event records in this chunk.
    ///
    /// This will emit constant pool and metadata events. Consumers probably
    /// want to exclude them by since they are handled specially.
    fn iter_event_records(
        &'reader self,
    ) -> Box<dyn Iterator<Item = Result<EventRecord<'a>>> + 'reader> {
        // Taking slice directly should be safe since we did parse it.
        let mut events_data = &self.data[ChunkHeader::HEADER_SIZE as _..];

        Box::new(std::iter::repeat(()).map_while(move |_| {
            if events_data.is_empty() {
                None
            } else {
                match EventRecord::parse(events_data) {
                    Ok((remaining, record)) => {
                        events_data = remaining;

                        Some(Ok(record))
                    }
                    Err(err) => Some(Err(err.into())),
                }
            }
        }))
    }

    /// Iterate constant pool events in this chunk.
    ///
    /// We start at the last chunk as annotated in the header and work our way
    /// backwards.
    fn iter_constant_pool_events(
        &'reader self,
    ) -> Box<dyn Iterator<Item = Result<ConstantPoolEvent<'a>>> + 'reader> {
        let mut offset = 0;
        let mut delta = self.header.constant_pool_position as i64;

        Box::new(std::iter::repeat(()).map_while(move |_| {
            if delta == 0 {
                None
            } else {
                offset += delta;

                match self.parse_constant_pool_event(offset as _) {
                    Ok((_, cp)) => {
                        delta = cp.header.delta;

                        Some(Ok(cp))
                    }
                    Err(err) => Some(Err(err.into())),
                }
            }
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const HEADER_HEX: &str = indoc! {"
        464c5200000200010000000000ce143d
        0000000000ce13dd0000000000003910
        177d216b5fbe159d0000000a6c74c4c5
        0000016d60607b51000000003b9aca00
        00000001
    "};

    #[test]
    fn parse_header() {
        let header = HEADER_HEX
            .bytes()
            .filter(|x| !x.is_ascii_whitespace())
            .collect::<Vec<_>>();

        let raw = hex::decode(&header).unwrap();

        let (s, header) = ChunkHeader::parse(&raw).unwrap();

        assert!(s.is_empty());
        assert_eq!(
            header,
            ChunkHeader {
                major: 2,
                minor: 1,
                chunk_size: 13505597,
                constant_pool_position: 13505501,
                metadata_position: 14608,
                nanoseconds_since_epoch: 1692545780012684701,
                duration_nanoseconds: 44769264837,
                start_ticks: 1569279998801,
                ticks_per_second: 1000000000,
                state_and_flags: 1,
            }
        );
    }
}
