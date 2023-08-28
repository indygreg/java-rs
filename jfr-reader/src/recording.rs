// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Multi chunk recording file handling.
//!
//! This module defines types that provide a high-level interface for reading
//! JFR data from multiple chunks. We refer to multi-chunk inputs as a
//! *recording*.

use crate::{
    chunk::{ChunkHeader, SliceReader},
    error::{Error, Result},
};
use std::io::{Read, Seek, SeekFrom};

/// Read JFR files.
pub struct FileReader<T: Read + Seek> {
    reader: T,
    offset: u64,
}

impl<T: Read + Seek> FileReader<T> {
    /// Construct an instance from a readable and seekable stream.
    ///
    /// It isn't performance critical for the stream to be buffered as we currently
    /// only support APIs for reading entire chunks. So buffering won't save
    /// that many system calls.
    pub fn from_stream(mut reader: T) -> Result<Self> {
        let offset = reader.stream_position()?;

        Ok(Self { reader, offset })
    }

    /// Read the data belonging to the next chunk from the underlying stream.
    ///
    /// Evaluates to [None] if it looks like we reached end of file.
    pub fn next_chunk_data(&mut self) -> Result<Option<Vec<u8>>> {
        self.reader.seek(SeekFrom::Start(self.offset))?;

        let mut buf = vec![0u8; ChunkHeader::HEADER_SIZE as usize];

        match self.reader.read(&mut buf)? {
            0 => {
                return Ok(None);
            }
            x if x == ChunkHeader::HEADER_SIZE as usize => {}
            x => {
                return Err(Error::Io(format!(
                    "read {} of {} bytes necessary to decode chunk header",
                    x,
                    ChunkHeader::HEADER_SIZE,
                )));
            }
        }

        // We let the nom parser guide us instead of codifying the logic here.
        let needed = match SliceReader::new(&buf) {
            Ok(_) => {
                // This should never happen, right?
                0
            }
            Err(Error::ParseIncomplete(nom::Needed::Size(needed))) => needed.into(),
            Err(err) => {
                return Err(err);
            }
        };

        buf.reserve_exact(needed);
        buf.extend(std::iter::repeat(0u8).take(needed));

        self.reader
            .read_exact(&mut buf[ChunkHeader::HEADER_SIZE as usize..])?;

        self.offset = self.reader.stream_position()?;

        Ok(Some(buf))
    }
}
