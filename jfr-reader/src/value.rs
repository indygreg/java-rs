// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    error::{Error, Result},
    metadata::ClassElement,
    primitive::Primitive,
    resolver::ConstantResolver,
};

/// An instance of a class with resolved values. Or in Java parlance an *Object*.
#[derive(Clone, Debug)]
pub struct Object<'a> {
    class: &'a ClassElement<'a>,
    fields: Vec<Value<'a>>,
}

impl<'a> Object<'a> {
    /// Construct a new instance.
    pub fn new(class: &'a ClassElement<'a>, fields: Vec<Value<'a>>) -> Self {
        Self { class, fields }
    }

    /// Obtain the class being described.
    pub fn class(&self) -> &ClassElement {
        self.class
    }

    /// Iterate over field values in this instance.
    pub fn iter_fields(&self) -> impl Iterator<Item = &Value<'a>> + '_ {
        self.fields.iter()
    }

    /// Obtain the field [Value] at a given field index.
    pub fn field_at(&self, index: usize) -> Option<&Value<'a>> {
        self.fields.get(index)
    }

    /// Resolve all constants references in this instance recursively.
    pub fn resolve_constants(mut self, constants: &impl ConstantResolver<'a>) -> Result<Self> {
        self.fields = self
            .fields
            .into_iter()
            .map(|v| v.resolve_constants(constants))
            .collect::<Result<Vec<_>>>()?;

        Ok(self)
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

    /// A constant pool reference that expanded to NULL.
    ///
    /// The special constant index 0 is used to represent NULL.
    ConstantPoolNull,

    /// An array of other values.
    Array(Vec<Value<'a>>),
}

impl<'a> Value<'a> {
    /// Obtain the inner [Object] if this is an object variant.
    pub fn as_object(&self) -> Option<&Object<'a>> {
        if let Value::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    /// Obtain the inner [Primitive] if this is a primitive variant.
    pub fn as_primitive(&self) -> Option<&Primitive<'a>> {
        if let Value::Primitive(p) = self {
            Some(p)
        } else {
            None
        }
    }

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
                        if constant_index == 0 {
                            Ok(Self::ConstantPoolNull)
                        } else {
                            Err(Error::EventParse(format!(
                                "constant pool entry {}:{} is missing",
                                class_id, constant_index
                            )))
                        }
                    }
                }
            }
            Self::ConstantPoolNull => Ok(Self::ConstantPoolNull),
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
