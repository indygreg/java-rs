// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Java Flight Recorder File Format Specification
//!
//! Wanted to learn about the JFR file format? You've come to the right place!
//!
//! # Chunks
//!
//! JFR data is generally emitted/consumed in units called *chunks*.
//!
//! Each chunk is self-contained and self-defines its tracked data.
//!
//! From a high level each chunk has a 68 byte header with magic `FLR\0`
//! followed by N variable length events. Each event has a header defining
//! its length and the integer event type being described. Each event
//! effectively consists of key-value pairs.
//!
//! You can walk events in a chunk by simply reading an event header and
//! skipping N bytes ahead to the next event. However, this yields little
//! to no useful information beyond counts/sizes of events by numeric type
//! ID.
//!
//! Chunks contain special events that define the types encountered in the
//! chunk and definitions of common/reused values and data structures. The
//! offsets to these special events are stored in the chunk header to facilitate
//! loading before event scanning.
//!
//! # Metadata
//!
//! The metadata is arguably the most important concept in JFR. The metadata
//! event holds the following:
//!
//! * A table of strings. (For string deduplication.)
//! * A list of *classes* / types that are referenced.
//!
//! The metadata effectively declares all the types/events that are present
//! in the chunk and how to parse their data.
//!
//! Each class defines its name, super type, simple/primitive type, annotations,
//! fields, and settings.
//!
//! Fields are type-specific properties further describing the class/event. For
//! example, a `threadSleep` event has a `long` field indicating how long it is
//! going to sleep. Think of fields as the well, fields, in a Java class or a
//! Rust struct because that's essentially what they are.
//!
//! Settings describe the settings of the JFR collector, like what the threshold
//! for slow lock recording was and whether to collect stack traces. These are
//! conceptually additional non-field metadata for the type.
//!
//! Annotations are common fields that the JFR framework can add to multiple
//! event types. Which thread produced the event, why the event is being recorded,
//! etc. Like settings they are additional metadata added by JFR collection.
//!
//! # Constant Pools
//!
//! Constants pools are special events defining commonly used/referenced data
//! values and complex data structures.
//!
//! An event type's value can be stored inline in the event or it can reference
//! a value in a constant pool. This allows commonly used values to be stored
//! once and referenced multiple times with minimal on-disk overhead. For
//! example, an event can reference a stack trace and the unique sequence of
//! frames in that stack is stored in a constants pool event so multiple events
//! can refer to the same stack without redundantly encoding the stack data.
//!
//! The constants pool is logically a mapping of class IDs to a mapping of
//! unique values for that type. The constants pool is logically spread across
//! N discrete events. The chunk header refers to the final constants pool
//! event. The constants pool / event header defines a negative integer offset
//! to the prior constants pool event. Chunk readers typically walk the events
//! backwards to populate the constants pool until they get to the initial
//! event.
//!
//! Storing the constants pool this way allows JFR writers to eagerly append new
//! constants pool entries to a file without having to buffer/preallocate space
//! for them or otherwise overwrite/copy file content.
//!
//! # Events
//!
//! Following the chunk header are variable length events. Each event defines
//! its size/length and the class type being described. Following these is
//! N-size bytes of values for each field in the referenced class type.
//!
//! The class type is a reference to a class defined in the metadata. Without
//! parsing the metadata it is impossible to interpret the field data in an
//! event record because the typing of each field is not known and values are
//! not self describing.
