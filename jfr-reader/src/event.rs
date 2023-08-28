// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR event handling.
//!
//! Events are data in chunks that aren't strings, elements, or constants.

use crate::{
    common::{leb128_i32, leb128_i64},
    error::{ParseResult, Result},
    resolver::{EventResolver, Value},
};
use nom::{bytes::streaming::take, error::context};
use std::borrow::Cow;

/// The event type ID referring to a metadata event.
pub const EVENT_TYPE_METADATA: i64 = 0;

/// The event type ID referring to a constant pool event.
pub const EVENT_TYPE_CONSTANT_POOL: i64 = 1;

/// Describes recognized event types.
pub enum EventType {
    Metadata,
    ConstantPool,
    Other(i64),
}

impl From<i64> for EventType {
    fn from(v: i64) -> Self {
        match v {
            EVENT_TYPE_METADATA => Self::Metadata,
            EVENT_TYPE_CONSTANT_POOL => Self::ConstantPool,
            v => Self::Other(v),
        }
    }
}

/// Generic header for events within a chunk.
#[derive(Clone, Debug, Default)]
pub struct EventHeader {
    pub size: i32,
    pub event_type: i64,
}

impl EventHeader {
    pub fn parse(s: &[u8]) -> ParseResult<Self> {
        let (s, size) = leb128_i32(s)?;
        let (s, event_type) = leb128_i64(s)?;

        Ok((s, Self { size, event_type }))
    }
}

/// Describes an event record inside a chunk.
pub trait ChunkEvent {
    /// Obtain the parsed event header.
    fn header(&self) -> &EventHeader;

    /// Obtain a reference to the full data for this event, inclusive of the header.
    fn event_data(&self) -> Result<&[u8]>;

    /// Obtain a reference to the fields data for this event.
    ///
    /// Effectively event_data() without the event header.
    fn fields_data(&self) -> Result<&[u8]>;

    /// Attempt to resolve the start and duration fields for this event.
    ///
    /// Most (all?) events have the start time and duration as their first two fields.
    ///
    /// This method attempts to resolve that pair of integers.
    ///
    /// No type checking is performed. So if the start time and duration are
    /// not actually the first two fields of this event, this function will
    /// likely succeed and return wrong integer values.
    fn start_duration(&self) -> Result<(i64, i64)> {
        let fields_data = self.fields_data()?;

        let (s, start_time) = context("reading assumed start time field", leb128_i64)(fields_data)?;
        let (_, duration) = context("reading assumed duration field", leb128_i64)(s)?;

        Ok((start_time, duration))
    }
}

/// Represents a minimally parsed chunk event and a reference to its data.
#[derive(Clone, Debug)]
pub struct EventRecord<'a> {
    pub header: EventHeader,

    /// Full event data, inclusive of header.
    event_data: Cow<'a, [u8]>,

    /// Offset of start of fields data within the event data.
    fields_data_offset: usize,
}

impl<'a> EventRecord<'a> {
    /// Parse an event record from chunk data.
    ///
    /// Will ensure the declared space for the event is available. But does not
    /// parse event fields data.
    pub fn parse(s: &'a [u8]) -> ParseResult<Self> {
        let (after_header, header) = context("parsing event header", EventHeader::parse)(s)?;

        let header_size = s.len() - after_header.len();

        let (s, event_data) = context("reading full event data", take(header.size as usize))(s)?;

        // Make sure all fields data is available.
        context("reading just fields data", take(header.size as usize))(event_data)?;

        Ok((
            s,
            Self {
                header,
                event_data: Cow::Borrowed(event_data),
                fields_data_offset: header_size,
            },
        ))
    }

    /// Whether this is a special event.
    ///
    /// Special events are typically not parsed by regular consumers.
    pub fn is_special_event(&self) -> bool {
        matches!(
            self.header.event_type,
            EVENT_TYPE_METADATA | EVENT_TYPE_CONSTANT_POOL
        )
    }

    /// Parse the event fields in this instance in a [Value] using an [EventResolver].
    pub fn resolve_value<'v: 'a, 'slf: 'v, 'r: 'v>(
        &'slf self,
        resolver: &'r EventResolver,
    ) -> Result<Value<'v>> {
        let (_, v) = resolver.parse_value(self.fields_data()?, self.header.event_type)?;

        Ok(v)
    }
}

impl<'a> ChunkEvent for EventRecord<'a> {
    fn header(&self) -> &EventHeader {
        &self.header
    }

    fn event_data(&self) -> Result<&[u8]> {
        Ok(&self.event_data)
    }

    fn fields_data(&self) -> Result<&[u8]> {
        Ok(&self.event_data[self.fields_data_offset..])
    }
}