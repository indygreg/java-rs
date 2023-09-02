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
//! [chunk::ChunkReader::iter_event_records] then call [event::EventRecord::resolve_value]
//! with the resolver to parse the event data into a [resolver::Value].

pub mod annotations;
pub mod chunk;
pub mod common;
pub mod constant_pool;
pub mod error;
pub mod event;
pub mod metadata;
pub mod recording;
pub mod resolver;
pub mod settings;
pub mod specification;
pub mod string_table;
pub mod value;
