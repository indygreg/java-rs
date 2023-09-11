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
use serde::{
    de::{
        value::StrDeserializer, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess,
        SeqAccess, Unexpected, VariantAccess, Visitor,
    },
    forward_to_deserialize_any, Deserialize, Deserializer,
};

/// Holds a constant's value.
pub enum ConstantValue<'a, 'r> {
    /// The constant evaluates to null.
    Null,
    /// The value of the constant.
    ///
    /// Can contain other constant pool references.
    Value(&'r Value<'a>),
    /// No constant known. Missing index in the constant pool.
    Missing,
}

/// Holds a constant's value with all constants resolved recursively.
pub enum ResolvedConstantValue<'a> {
    Null,
    Value(Result<Value<'a>>),
    Missing,
}

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

/// A deserializer for [Object] instances.
struct ObjectDeserializer<'de, 'a: 'de, CR>
where
    CR: ConstantResolver<'a>,
{
    object: &'de Object<'a>,
    constants: &'de CR,
    field_index: usize,
}

impl<'de, 'a: 'de, CR> MapAccess<'de> for ObjectDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(field) = self.object.class().fields.get(self.field_index) {
            let key = field.name.as_ref();
            let key: StrDeserializer<Self::Error> = key.into_deserializer();
            let key: K::Value = seed.deserialize(key)?;

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let field_value = self.object.field_at(self.field_index).unwrap();

        let value = seed.deserialize(ValueDeserializer {
            value: field_value,
            constants: self.constants,
        })?;
        self.field_index += 1;

        Ok(value)
    }

    // TODO implement size_hint()?
}

// TODO consider exposing a variant for null. Maybe as a primitive variant?
// Reconcile with Primitive::NullString.
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
                    ResolvedConstantValue::Null => Ok(Self::ConstantPoolNull),
                    ResolvedConstantValue::Value(res) => res,
                    ResolvedConstantValue::Missing => Err(Error::EventParse(format!(
                        "constant pool entry {}:{} is missing",
                        class_id, constant_index
                    ))),
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

    /// Deserialize an instance to a type.
    pub fn deserialize<'de, 'slf: 'de, 'cr: 'de, T>(
        &'slf self,
        constants: &'cr impl ConstantResolver<'a>,
    ) -> Result<T>
    where
        T: Deserialize<'de>,
    {
        Ok(T::deserialize(ValueDeserializer::new(self, constants))?)
    }

    /// Deserialize into an enum.
    ///
    /// The target enum must have variant names matching the leaf class
    /// name. And the variant must be a newtype variant. e.g. a
    /// `jdk.ThreadPark` JFR event will require an enum with
    /// a `ThreadPark(ThreadPark)` variant, where `ThreadPark` is a
    /// struct.
    ///
    /// If the target enum does not have a variant matching the class
    /// name, an error occurs.
    pub fn deserialize_enum<'de, 'slf: 'de, 'cr: 'de, T>(
        &'slf self,
        constants: &'cr impl ConstantResolver<'a>,
    ) -> Result<T>
    where
        T: Deserialize<'de>,
    {
        Ok(T::deserialize(EventsEnumDeserializer::new(
            self, constants,
        ))?)
    }
}

/// A deserializer for an array of values.
struct ArrayDeserializer<'de, 'a: 'de, CR>
where
    CR: ConstantResolver<'a>,
{
    array: &'de Vec<Value<'a>>,
    constants: &'de CR,
    index: usize,
}

impl<'de, 'a: 'de, CR> SeqAccess<'de> for ArrayDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = &self.array.get(self.index) {
            let deserializer = ValueDeserializer {
                value,
                constants: self.constants,
            };
            let value = seed.deserialize(deserializer)?;
            self.index += 1;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.array.len() - self.index)
    }
}

/// A deserializer for a [Value].
pub struct ValueDeserializer<'de, 'a: 'de, CR>
where
    CR: ConstantResolver<'a>,
{
    value: &'de Value<'a>,
    constants: &'de CR,
}

impl<'de, 'a: 'de, CR> ValueDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    /// Construct an instance from a [Value].
    pub fn new(value: &'de Value<'a>, constants: &'de CR) -> Self {
        Self { value, constants }
    }
}

impl<'de, 'a: 'de, CR> Deserializer<'de> for ValueDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    type Error = Error;

    // TODO implement deserialize_* methods instead of falling back to any().
    // We know what we're expecting to deserialize so we shouldn't fall back
    // to any mode.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Primitive(p) => match p {
                Primitive::Boolean(v) => visitor.visit_bool(*v),
                Primitive::Byte(v) => visitor.visit_i8(*v),
                Primitive::Short(v) => visitor.visit_i16(*v),
                Primitive::Integer(v) => visitor.visit_i32(*v),
                Primitive::Long(v) => visitor.visit_i64(*v),
                Primitive::Float(v) => visitor.visit_f32(*v),
                Primitive::Double(v) => visitor.visit_f64(*v),
                Primitive::Character(v) => visitor.visit_char(*v),
                Primitive::NullString => visitor.visit_none(),
                Primitive::String(v) => visitor.visit_borrowed_str(v.as_ref()),
                Primitive::StringConstantPool(index) => match self.constants.get_string(*index) {
                    ConstantValue::Null => visitor.visit_none(),
                    ConstantValue::Value(v) => {
                        let de = ValueDeserializer {
                            value: v,
                            constants: self.constants,
                        };
                        Self::deserialize_any(de, visitor)
                    }
                    ConstantValue::Missing => Err(Error::Deserialize(format!(
                        "string constant {} not found",
                        index
                    ))),
                },
            },
            Value::Object(o) => visitor.visit_map(ObjectDeserializer {
                object: o,
                constants: self.constants,
                field_index: 0,
            }),
            Value::Array(array) => visitor.visit_seq(ArrayDeserializer {
                array,
                constants: self.constants,
                index: 0,
            }),
            Value::ConstantPool {
                class_id,
                constant_index,
            } => match self.constants.get(*class_id, *constant_index) {
                ConstantValue::Null => visitor.visit_none(),
                ConstantValue::Value(v) => {
                    let de = ValueDeserializer {
                        value: v,
                        constants: self.constants,
                    };
                    Self::deserialize_any(de, visitor)
                }
                ConstantValue::Missing => Err(Error::Deserialize(format!(
                    "constant pool {}:{} not found",
                    class_id, constant_index
                ))),
            },
            Value::ConstantPoolNull => visitor.visit_none(),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Primitive(Primitive::NullString) => visitor.visit_none(),
            Value::ConstantPool {
                class_id,
                constant_index,
            } => match self.constants.get(*class_id, *constant_index) {
                ConstantValue::Null => visitor.visit_none(),
                ConstantValue::Value(v) => {
                    let de = ValueDeserializer {
                        value: v,
                        constants: self.constants,
                    };
                    visitor.visit_some(de)
                }
                ConstantValue::Missing => Err(Error::Deserialize(format!(
                    "constant pool {}:{} not found",
                    class_id, constant_index
                ))),
            },
            _ => visitor.visit_some(self),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

/// A specialized deserializer for enums representing event types.
///
/// The enum being deserialized should have variant names matching the
/// leaf class name of JFR events. The deserializer will automatically
/// attempt to deserialize the variant whose name matches the Value's
/// class name.
///
/// Currently only works on [Value::Object] variants.
pub struct EventsEnumDeserializer<'de, 'a: 'de, CR>
where
    CR: ConstantResolver<'a>,
{
    value: &'de Value<'a>,
    constants: &'de CR,
}

impl<'de, 'a: 'de, CR> EventsEnumDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    /// Construct a new instance from a [Value] and [ConstantResolver].
    pub fn new(value: &'de Value<'a>, constants: &'de CR) -> Self {
        Self { value, constants }
    }
}

impl<'de, 'a: 'de, CR> EnumAccess<'de> for EventsEnumDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> std::result::Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let o = self
            .value
            .as_object()
            .ok_or_else(|| Error::Deserialize("expected Object variant of Value".to_string()))?;

        let name = o.class.name.as_ref();

        let variant = if let Some((_, leaf)) = name.rsplit_once('.') {
            leaf
        } else {
            name
        };

        let variant: StrDeserializer<Self::Error> = variant.into_deserializer();
        let variant: V::Value = seed.deserialize(variant)?;

        Ok((variant, self))
    }
}

impl<'de, 'a: 'de, CR> VariantAccess<'de> for EventsEnumDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    type Error = Error;

    fn unit_variant(self) -> std::result::Result<(), Self::Error> {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> std::result::Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(ValueDeserializer::new(self.value, self.constants))
    }

    fn tuple_variant<V>(self, _: usize, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::TupleVariant,
            &"newtype variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _: &'static [&'static str],
        _: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::StructVariant,
            &"newtype variant",
        ))
    }
}

impl<'de, 'a: 'de, CR> Deserializer<'de> for EventsEnumDeserializer<'de, 'a, CR>
where
    CR: ConstantResolver<'a>,
{
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Deserialize(
            "deserialize_any not supported".to_string(),
        ))
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any option
    }
}
