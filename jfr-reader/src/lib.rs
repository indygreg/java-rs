// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Java Flight Recorder reader.
//!
//! This crate aims to implement a no-compromises reader for Java Flight Recorder
//! (JFR) data. By *no compromises* we mean:
//!
//! * Is able to fully decode every piece of data in JFR files.
//! * Is able to do so safely - no panics / all errors can be caught.
//! * Is able to do so efficiently. No slower than the official JFR reader. Ideally
//!   faster. Both in terms of wall time and CPU cycles.
//! * Offers a flexible and useful API that allows consumers to access JFR
//!   data easily without having to fully understand the internals of the JFR
//!   format.
//!
//! This crate does not currently succeed at all of these goals!
//!
//! See [specification] for an overview of the JFR file format. The
//! documentation of modules and types therein is generally pretty thorough.
//!
//! JFR files consist of self-contained units called *chunks*. The
//! [chunk::ChunkReader] trait defines a common interface for reading from
//! chunks. The [chunk::SliceReader] provides an implementation to read chunks
//! from a `&[u8]`.
//!
//! If you want to read chunks from a file, see [recording::FileReader]. It gives
//! you an API for resolving chunk data. You can then construct a
//! [chunk::SliceReader] to read from the chunk.
//!
//! [chunk::SliceReader] exposes some APIs to read from the chunk, but they are
//! exceptionally low level and probably not useful by themselves. You should obtain
//! an [resolver::EventResolver] via [chunk::ChunkReader::resolver] on the
//! [chunk::SliceReader]. This type is used to interpret event data and turn
//! it into stronger types data structures.
//!
//! Once you have a [resolver::EventResolver] you can call
//! [chunk::ChunkReader::iter_event_records] then call [chunk_event::EventRecord::resolve_object]
//! with the resolver to parse the event data into a [resolver::Value].

pub mod annotations;
pub mod chunk;
pub mod chunk_event;
pub mod common;
pub mod constant_pool;
pub mod error;
pub mod event;
pub mod metadata;
#[cfg(feature = "metadata-xml-derive")]
pub mod metadata_xml;
pub mod primitive;
pub mod recording;
pub mod resolver;
pub mod settings;
pub mod specification;
pub mod string_table;
pub mod types;
pub mod value;

#[cfg(test)]
mod test {
    use super::*;
    use crate::chunk::ChunkReader;

    #[test]
    fn read_intellij() -> error::Result<()> {
        let input = include_bytes!("../testdata/intellij.jfr.zst");
        let input = zstd::decode_all(input.as_slice()).unwrap();

        let mut fr = recording::FileReader::from_stream(std::io::Cursor::new(input.as_slice()))?;

        // A couple events fail to deserialize due to use of u64 in type definition
        // but presence of -1 as stored value.
        let known_bad = vec![(0, 2584), (0, 2615)];

        for (chunk_index, chunk) in fr.all_chunks()?.into_iter().enumerate() {
            let reader = chunk::SliceReader::new(&chunk)?.1;

            let resolver = reader.resolver()?;
            let cpv = resolver.constant_pool_values()?;

            for (event_index, er) in reader.iter_event_records().enumerate() {
                let er = er?;

                if er.is_special_event() {
                    continue;
                }

                let v = er.resolve_value(&resolver)?;

                if v.deserialize_enum::<types::openjdk17::Events>(&cpv)
                    .is_err()
                {
                    assert!(
                        known_bad.contains(&(chunk_index, event_index)),
                        "event {}:{} is known bad",
                        chunk_index,
                        event_index
                    );
                }
            }
        }

        Ok(())
    }
}
