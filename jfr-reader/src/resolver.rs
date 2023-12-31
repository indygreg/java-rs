// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Event resolution functionality.
//!
//! Events as encoded in chunks are exceptionally low level and need to use
//! class metadata and constants pools to resolve event values. This module
//! contains the logic for doing this.

use crate::{
    chunk::ChunkHeader,
    common::{leb128_i32, leb128_i64},
    constant_pool::ConstantPoolEvent,
    error::{Error, ParseResult, Result},
    event::GenericEvent,
    metadata::{ClassElement, FieldElement, Metadata},
    primitive::Primitive,
    value::{ConstantValue, Object, ResolvedConstantValue, Value},
};
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};
use rustc_hash::FxHashMap;

/// Describes an entity that can resolve constants.
pub trait ConstantResolver<'a>: Sized {
    /// Get the value of a constant.
    fn get(&self, class_id: i64, index: i64) -> ConstantValue<'a, '_>;

    /// Get the value of a constant and recursively expand constant references.
    fn get_recursive(&self, class_id: i64, index: i64) -> ResolvedConstantValue<'a> {
        match self.get(class_id, index) {
            ConstantValue::Null => ResolvedConstantValue::Null,
            ConstantValue::Missing => ResolvedConstantValue::Missing,
            ConstantValue::Value(v) => {
                ResolvedConstantValue::Value(v.clone().resolve_constants(self))
            }
        }
    }

    /// Get the value of a string constant.
    fn get_string(&self, index: i64) -> ConstantValue<'a, '_>;
}

/// Holds resolved values in the constants pool.
pub struct ConstantPoolValues<'a> {
    inner: FxHashMap<i64, FxHashMap<i64, Value<'a>>>,
    /// The class ID for java.lang.String.
    ///
    /// Stored to facilitate efficient lookups of constant pool references
    /// to strings.
    string_class_id: Option<i64>,
}

impl<'a> ConstantResolver<'a> for ConstantPoolValues<'a> {
    fn get(&self, class_id: i64, index: i64) -> ConstantValue<'a, '_> {
        match self.inner.get(&class_id) {
            Some(x) => match x.get(&index) {
                Some(v) => ConstantValue::Value(v),
                None if index == 0 => ConstantValue::Null,
                None => ConstantValue::Missing,
            },
            None if index == 0 => ConstantValue::Null,
            None => ConstantValue::Missing,
        }
    }

    fn get_string(&self, index: i64) -> ConstantValue<'a, '_> {
        if let Some(class_id) = self.string_class_id {
            self.get(class_id, index)
        } else {
            ConstantValue::Missing
        }
    }
}

/// Entity for resolving time from chunk header metadata.
///
/// Events within a chunk express time in terms of *ticks*. These ticks
/// need to be combined with metadata from the chunk header to resolve
/// into wall time and units of time.
///
/// This type exists to facilitate those conversions.
#[derive(Clone, Debug)]
pub struct TimeResolver {
    start_epoch_nanoseconds: i64,
    start_ticks: u64,
    ticks_per_second: u64,
    start_date_time: DateTime<FixedOffset>,
}

impl TimeResolver {
    /// Construct an instance from a [ChunkHeader] and [Metadata] instance.
    pub fn new(header: &ChunkHeader, metadata: &Metadata) -> Result<Self> {
        let start_date_time = metadata
            .root
            .region
            .fixed_offset()?
            .timestamp_nanos(header.nanoseconds_since_epoch as _);

        Ok(Self {
            start_epoch_nanoseconds: header.nanoseconds_since_epoch as i64,
            start_ticks: header.start_ticks,
            ticks_per_second: header.ticks_per_second,
            start_date_time,
        })
    }

    /// Number of nanoseconds between the specified ticks value and the chunk start.
    #[inline]
    pub fn chunk_start_delta_nanoseconds(&self, ticks: i64) -> i64 {
        let delta_ticks = ticks - self.start_ticks as i64;

        delta_ticks / (self.ticks_per_second / 1_000_000_000) as i64
    }

    /// The time between ticks and chunk start expressed as a [Duration].
    pub fn chunk_start_delta_duration(&self, ticks: i64) -> Duration {
        Duration::nanoseconds(self.chunk_start_delta_nanoseconds(ticks))
    }

    /// The nanoseconds since UNIX epoch given a ticks count.
    pub fn epoch_nanoseconds(&self, ticks: i64) -> i64 {
        self.start_epoch_nanoseconds + self.chunk_start_delta_nanoseconds(ticks)
    }

    /// Obtain a [DateTime] for a ticks value preserving the timezone from the metadata.
    pub fn date_time(&self, ticks: i64) -> DateTime<FixedOffset> {
        self.start_date_time + self.chunk_start_delta_duration(ticks)
    }

    /// Obtain a [DateTime] for a ticks value having the UTC timezone.
    pub fn date_time_utc(&self, ticks: i64) -> DateTime<Utc> {
        self.date_time(ticks).with_timezone(&Utc)
    }

    /// Obtain the time between 2 ticks in nanoseconds.
    #[inline]
    pub fn delta_nanoseconds(&self, start_ticks: i64, end_ticks: i64) -> i64 {
        let delta_ticks = end_ticks - start_ticks;
        delta_ticks / (self.ticks_per_second / 1_000_000_000) as i64
    }

    /// Obtain the amount of time between 2 tick values as a [Duration].
    pub fn delta_duration(&self, start_ticks: i64, end_ticks: i64) -> Duration {
        Duration::nanoseconds(self.delta_nanoseconds(start_ticks, end_ticks))
    }
}

/// Entity for resolving values from event records.
///
/// This struct contains the logic for turning bag-of-bytes events into higher
/// level types. It does this by using the class/typing definitions in the
/// [Metadata] event to interpret the attributes/fields/values inside the
/// events.
///
/// This type enforces pay-for-what-you-use principles. e.g. it is possible to
/// resolve just an event's/class's field values without resolving annotations,
/// settings, or referenced values in the constants pool. It is possible to do
/// all of this without parsing the constants pool at all. This enables consumers
/// to filter on event field values without having to pay additional costs.
pub struct EventResolver<'a> {
    classes: FxHashMap<i64, ClassElement<'a>>,
    constant_pools: Vec<ConstantPoolEvent<'a>>,
    primitive_parsers: FxHashMap<i64, fn(&'a [u8]) -> ParseResult<'a, Primitive<'a>>>,
    time_resolver: TimeResolver,
}

impl<'a> EventResolver<'a> {
    /// Construct an instance from metadata and lightly parsed constants pools.
    pub fn new(
        chunk_header: &ChunkHeader,
        metadata: Metadata<'a>,
        constant_pools: impl Iterator<Item = ConstantPoolEvent<'a>>,
    ) -> Result<Self> {
        let time_resolver = TimeResolver::new(chunk_header, &metadata)?;

        let classes = FxHashMap::from_iter(
            metadata
                .root
                .metadata
                .classes
                .into_iter()
                .map(|x| (x.id, x)),
        );

        let constant_pools = constant_pools.collect::<Vec<_>>();

        let mut primitive_parsers = FxHashMap::default();

        for class in classes.values() {
            if let Some(parser) = Primitive::resolve_parser(class.name.as_ref()) {
                primitive_parsers.insert(class.id, parser);
            }
        }

        Ok(Self {
            classes,
            constant_pools,
            primitive_parsers,
            time_resolver,
        })
    }

    pub fn get_class(&self, id: i64) -> Option<&ClassElement<'a>> {
        self.classes.get(&id)
    }

    /// Resolve the name of the class having the specified ID.
    pub fn class_name(&self, id: i64) -> Option<&str> {
        self.classes.get(&id).map(|c| c.name.as_ref())
    }

    /// Resolve the class ID for the class having the specified name.
    pub fn class_id(&self, name: &str) -> Option<i64> {
        self.classes
            .iter()
            .find_map(|(k, v)| if v.name == name { Some(*k) } else { None })
    }

    /// Obtain class IDs and their names.
    pub fn class_ids_and_names(&self) -> impl Iterator<Item = (i64, &str)> + '_ {
        self.classes.iter().map(|(k, v)| (*k, v.name.as_ref()))
    }

    /// Obtain a data structure allowing retrieval of resolved constant pool values.
    pub fn constant_pool_values(&self) -> Result<ConstantPoolValues<'_>> {
        let mut inner = FxHashMap::<i64, FxHashMap<i64, Value>>::default();

        for e in &self.constant_pools {
            for (class_id, values) in e.resolve_constants(self)? {
                let entry = inner.entry(class_id).or_default();

                for (index, v) in values {
                    entry.insert(index, v);
                }
            }
        }

        let string_class_id = self.class_id("java.lang.String");

        Ok(ConstantPoolValues {
            inner,
            string_class_id,
        })
    }

    /// Obtain the [TimeResolver] for this instance.
    pub fn time_resolver(&self) -> &TimeResolver {
        &self.time_resolver
    }

    /// Parse event fields data into a [GenericEvent].
    pub fn parse_event<'slf, 'cr, CR: ConstantResolver<'slf>>(
        &'slf self,
        s: &'a [u8],
        class_id: i64,
        cr: &'cr CR,
    ) -> Result<(&[u8], GenericEvent<'slf, 'cr, CR>)> {
        let (s, o) = self.parse_event_object(s, class_id)?;

        let res = GenericEvent::new(o, cr);

        Ok((s, res))
    }

    /// Parse event fields data into a [Value].
    ///
    /// The class ID of the event class to resolve is passed in. The value
    /// should come from the event's header in the chunk data.
    pub fn parse_event_value(&self, s: &'a [u8], class_id: i64) -> Result<(&[u8], Value<'_>)> {
        self.parse_value(s, class_id)
    }

    /// Parse event fields data into an [Object].
    ///
    /// Like [Self::parse_event_value()] but downcasts to an [Object].
    pub fn parse_event_object(&self, s: &'a [u8], class_id: i64) -> Result<(&[u8], Object<'_>)> {
        let (s, v) = self.parse_value(s, class_id)?;

        if let Value::Object(o) = v {
            Ok((s, o))
        } else {
            Err(Error::EventParse(
                "parsed value is not an Object (this should not happen)".to_string(),
            ))
        }
    }

    /// Resolve a dynamic value from an input buffer.
    ///
    /// The class ID of the value to resolve is passed in. This function looks up
    /// the class definition and parses the class's fields recursively until the
    /// value is fully constructed.
    ///
    /// This function does not concern itself with annotations, settings, or resolving
    /// constant pool references. The goal is to recursively interpret the passed in
    /// slice so all referenced data is captured.
    pub(crate) fn parse_value(
        &self,
        mut s: &'a [u8],
        class_id: i64,
    ) -> Result<(&'a [u8], Value<'_>)> {
        // Use a cached lookup table of parsers for common classes so we can avoid both the class
        // lookup (fast) and the string compare to locate the parser function (slow).
        // TODO support registering additional parser functions to make this fully generic.
        if let Some(parser) = self.primitive_parsers.get(&class_id) {
            let (remaining, v) = parser(s)?;

            return Ok((remaining, Value::Primitive(v)));
        }

        let class = self
            .get_class(class_id)
            .ok_or(Error::ClassNotFound(class_id))?;

        let mut fields = Vec::with_capacity(class.fields.len());

        // The value consists of attributes/fields defined in the order from their
        // class definition.
        for field in class.fields.iter() {
            let (remaining, v) = if field.is_array_type() {
                self.parse_field_array(s, field)?
            } else {
                self.parse_field_single(s, field)?
            };

            s = remaining;
            fields.push(v);
        }

        let v = Value::Object(Object::new(class, fields));

        Ok((s, v))
    }

    /// Resolve a value from a field.
    ///
    /// Classes are composed of fields. This function decodes a single field within
    /// a class.
    fn parse_field_single(
        &self,
        s: &'a [u8],
        field: &FieldElement<'a>,
    ) -> Result<(&'a [u8], Value<'_>)> {
        // This seems to always be "true" if present. Don't bother checking it.
        if field.constant_pool.is_some() {
            let (s, constant_index) = leb128_i64(s).map_err(Error::from)?;

            let v = Value::ConstantPool {
                class_id: field.type_id,
                constant_index,
            };

            Ok((s, v))
        } else {
            self.parse_value(s, field.type_id)
        }
    }

    /// Resolve an array field.
    ///
    /// This is a special variant of [Self::parse_field_array] that should be called when
    /// the field is an array.
    fn parse_field_array(
        &self,
        s: &'a [u8],
        field: &FieldElement<'a>,
    ) -> Result<(&'a [u8], Value<'_>)> {
        let (mut s, array_length) = leb128_i32(s).map_err(Error::from)?;

        let mut els = Vec::with_capacity(array_length as _);

        for _ in 0..array_length {
            let (remaining, v) = self.parse_field_single(s, field)?;
            s = remaining;
            els.push(v);
        }

        Ok((s, Value::Array(els)))
    }
}
