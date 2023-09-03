// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Event resolutions functionality.
//!
//! Events as encoded in chunks are exceptionally low level and need to use
//! class metadata and constants pools to resolve event values. This module
//! contains the logic for doing this.

use crate::{
    common::{leb128_i32, leb128_i64},
    constant_pool::ConstantPoolEvent,
    error::{Error, ParseResult, Result},
    metadata::{ClassElement, FieldElement, Metadata},
    value::Primitive,
};
use rustc_hash::FxHashMap;

/// Describes an entity that can resolve constants.
pub trait ConstantResolver<'a>: Sized {
    /// Get the value of a constant.
    fn get(&self, class_id: i64, index: i64) -> Option<&Value<'a>>;

    /// Get the value of a constant and recursively expand constant references.
    fn get_recursive(&self, class_id: i64, index: i64) -> Option<Result<Value<'a>>> {
        self.get(class_id, index)
            .map(|v| v.clone().resolve_constants(self))
    }
}

/// Holds resolved values in the constants pool.
pub struct ConstantPoolValues<'a> {
    inner: FxHashMap<i64, FxHashMap<i64, Value<'a>>>,
}

impl<'a> ConstantResolver<'a> for ConstantPoolValues<'a> {
    fn get(&self, class_id: i64, index: i64) -> Option<&Value<'a>> {
        self.inner.get(&class_id).and_then(|x| x.get(&index))
    }
}

/// An instance of a class with resolved values. Or in Java parlance an *Object*.
#[derive(Clone, Debug)]
pub struct Object<'a> {
    class: &'a ClassElement<'a>,
    fields: Vec<Value<'a>>,
}

impl<'a> Object<'a> {
    /// Obtain the class being described.
    pub fn class(&self) -> &ClassElement {
        self.class
    }

    /// Iterate over field values in this instance.
    pub fn iter_fields(&self) -> impl Iterator<Item = &Value<'a>> + '_ {
        self.fields.iter()
    }
}

/// Enumeration for different value types.
#[derive(Clone, Debug)]
pub enum Value<'a> {
    /// A JVM primitive/built-in value.
    ///
    /// Represents simple/common values.
    Primitive(Primitive<'a>),

    /// A generic, untyped object.
    Object(Object<'a>),

    /// A reference to a constant in the constants pool.
    ///
    /// The value can be found in the `class_id` type in slot `constant_index`.
    ConstantPool { class_id: i64, constant_index: i64 },

    /// A reference to a constant in the constants pool. But that entry is not found.
    ConstantPoolMissing { class_id: i64, constant_index: i64 },

    /// An array of other values.
    Array(Vec<Value<'a>>),
}

impl<'a> Value<'a> {
    /// Resolve all constants references in this value recursively.
    ///
    /// The resulting Value should not have any instances of the Value::ConstantPool variant.
    pub fn resolve_constants(self, constants: &impl ConstantResolver<'a>) -> Result<Self> {
        match self {
            Self::Primitive(v) => Ok(Self::Primitive(v)),
            Self::Object(mut o) => {
                o.fields = o
                    .fields
                    .into_iter()
                    .map(|v| v.resolve_constants(constants))
                    .collect::<Result<Vec<_>>>()?;

                Ok(Self::Object(o))
            }
            Self::ConstantPool {
                class_id,
                constant_index,
            } => {
                // Resolved value could itself have constants. So we need to resolve recursively.
                match constants.get_recursive(class_id, constant_index) {
                    Some(res) => res,
                    None => {
                        //println!("Missing constants pool: {} {}", class_id, constant_index);

                        Ok(Self::ConstantPoolMissing {
                            class_id,
                            constant_index,
                        })
                    }
                }
            }
            Self::ConstantPoolMissing {
                class_id,
                constant_index,
            } => Ok(Self::ConstantPoolMissing {
                class_id,
                constant_index,
            }),
            Self::Array(a) => {
                let a = a
                    .into_iter()
                    .map(|x| x.resolve_constants(constants))
                    .collect::<Result<Vec<_>>>()?;

                Ok(Self::Array(a))
            }
        }
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
}

impl<'a> EventResolver<'a> {
    /// Construct an instance from metadata and lightly parsed constants pools.
    pub fn new(
        metadata: Metadata<'a>,
        constant_pools: impl Iterator<Item = ConstantPoolEvent<'a>>,
    ) -> Result<Self> {
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
        })
    }

    pub fn get_class(&self, id: i64) -> Option<&ClassElement<'a>> {
        self.classes.get(&id)
    }

    /// Resolve the name of the class having the specified ID.
    pub fn class_name(&self, id: i64) -> Option<&str> {
        self.classes.get(&id).map(|c| c.name.as_ref())
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

        Ok(ConstantPoolValues { inner })
    }

    /// Resolve a dynamic value from an input buffer.
    ///
    /// The class ID of the value to resolve is passed in. This function looks up
    /// the class definition and parses the class's fields recursively until the
    /// value is fully constructed.
    ///
    /// This function does not concern itself with annotations, settings, or resolving
    /// constant pool references.
    pub fn parse_value(&self, mut s: &'a [u8], class_id: i64) -> Result<(&'a [u8], Value<'_>)> {
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

        let v = Value::Object(Object { class, fields });

        Ok((s, v))
    }

    /// Resolve a value from a field.
    ///
    /// Classes are composed of fields. This function decodes a single field within
    /// a class.
    pub fn parse_field_single(
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
    pub fn parse_field_array(
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
