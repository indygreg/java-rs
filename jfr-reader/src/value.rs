// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    error::{Error, Result},
    metadata::{ClassElement, FieldElement},
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
pub enum ConstantValue<'chunk, 'resolver> {
    /// The constant evaluates to null.
    Null,
    /// The value of the constant.
    ///
    /// Can contain other constant pool references.
    Value(&'resolver Value<'chunk, 'resolver>),
    /// No constant known. Missing index in the constant pool.
    Missing,
}

/// Holds a constant's value with all constants resolved recursively.
pub enum ResolvedConstantValue<'chunk, 'resolver> {
    Null,
    Value(Result<Value<'chunk, 'resolver>>),
    Missing,
}

/// Holds a constant's value mapped to an arbitrary different type.
pub enum ConstantValueMapped<T> {
    Null,
    Value(Result<T>),
    Missing,
}

/// An instance of a class with resolved values. Or in Java parlance an *Object*.
#[derive(Clone, Debug)]
pub struct Object<'resolver, 'chunk: 'resolver> {
    class: &'resolver ClassElement<'chunk>,
    fields: Vec<Value<'chunk, 'resolver>>,
}

impl<'resolver, 'chunk: 'resolver> Object<'resolver, 'chunk> {
    /// Construct a new instance.
    pub fn new(
        class: &'resolver ClassElement<'chunk>,
        fields: Vec<Value<'chunk, 'resolver>>,
    ) -> Self {
        Self { class, fields }
    }

    /// Obtain the class being described.
    pub fn class(&self) -> &'resolver ClassElement {
        self.class
    }

    /// Iterate fields and their respective values.
    pub fn iter_fields_and_values(
        &self,
    ) -> impl Iterator<Item = (&'resolver FieldElement<'chunk>, &Value<'chunk, 'resolver>)> + '_
    {
        self.class.fields.iter().zip(self.fields.iter())
    }

    /// Iterate over field values in this instance.
    pub fn iter_fields(&self) -> impl Iterator<Item = &Value<'chunk, 'resolver>> + '_ {
        self.fields.iter()
    }

    /// Obtain the field [Value] at a given field index.
    pub fn field_at(&self, index: usize) -> Option<&Value<'chunk, 'resolver>> {
        self.fields.get(index)
    }

    /// Resolve all constants references in this instance recursively.
    pub fn resolve_constants<'cr: 'resolver>(
        mut self,
        constants: &'cr impl ConstantResolver<'chunk>,
    ) -> Result<Self> {
        self.fields = self
            .fields
            .into_iter()
            .map(|v| v.resolve_constants(constants))
            .collect::<Result<Vec<_>>>()?;

        Ok(self)
    }
}

/// A deserializer for [Object] instances.
struct ObjectDeserializer<'de, 'chunk: 'de, 'resolver: 'de, CR>
where
    CR: ConstantResolver<'chunk>,
{
    object: &'de Object<'resolver, 'chunk>,
    constants: &'de CR,
    field_index: usize,
}

impl<'de, 'chunk: 'de, 'resolver: 'de, CR> MapAccess<'de>
    for ObjectDeserializer<'de, 'chunk, 'resolver, CR>
where
    CR: ConstantResolver<'chunk>,
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
pub enum Value<'chunk, 'resolver> {
    /// A JVM primitive/built-in value.
    ///
    /// Represents simple/common values.
    Primitive(Primitive<'chunk>),

    /// A generic, untyped object.
    Object(Object<'resolver, 'chunk>),

    /// A reference to a constant in the constants pool.
    ///
    /// The value can be found in the `class_id` type in slot `constant_index`.
    ConstantPool { class_id: i64, constant_index: i64 },

    /// A constant pool reference that expanded to NULL.
    ///
    /// The special constant index 0 is used to represent NULL.
    ConstantPoolNull,

    /// An array of other values.
    Array(Vec<Value<'chunk, 'resolver>>),
}

impl<'chunk, 'resolver> Value<'chunk, 'resolver> {
    /// Obtain the inner [Object] if this is an object variant.
    pub fn as_object(&self) -> Option<&Object<'resolver, 'chunk>> {
        if let Value::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    /// Obtain the inner [Primitive] if this is a primitive variant.
    pub fn as_primitive(&self) -> Option<&Primitive<'chunk>> {
        if let Value::Primitive(p) = self {
            Some(p)
        } else {
            None
        }
    }

    /// Resolve all constants references in this value recursively.
    ///
    /// The resulting Value should not have any instances of the Value::ConstantPool variant.
    pub fn resolve_constants<'cr: 'resolver>(
        self,
        constants: &'cr impl ConstantResolver<'chunk>,
    ) -> Result<Self> {
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
        constants: &'cr impl ConstantResolver<'chunk>,
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
        constants: &'cr impl ConstantResolver<'chunk>,
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
struct ArrayDeserializer<'de, 'chunk: 'de, 'resolver: 'de, CR>
where
    CR: ConstantResolver<'chunk>,
{
    array: &'de Vec<Value<'chunk, 'resolver>>,
    constants: &'de CR,
    index: usize,
}

impl<'de, 'chunk: 'de, 'resolver: 'de, CR> SeqAccess<'de>
    for ArrayDeserializer<'de, 'chunk, 'resolver, CR>
where
    CR: ConstantResolver<'chunk>,
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
///
/// # Note on Missing Constant Pool Entries
///
/// Sometimes JFR events have references to constant pool entries
/// that don't exist. The CP index 0 is reserved to express null
/// and it generally doesn't exist. But sometimes we see missing
/// non-0 indices. We're not sure why. (Commonly missing constants
/// are java.lang.Thread classes.)
///
/// It appears that official JFR readers treat a missing CP index
/// as null. Although they overload null to mean both the intentional
/// null value and a missing value. So it isn't clear if they actually
/// intended to allow missing constant pool references.
///
/// We mimic the behavior of official JFR readers by interpreting
/// missing constant pool values as null / None. This is arguably
/// not correct. But it is the most user-friendly behavior. If we
/// wanted to be more strict, we could potentially have a flag to
/// control behavior.
pub struct ValueDeserializer<'de, 'chunk: 'de, 'resolver: 'de, 'cr: 'de, CR>
where
    CR: ConstantResolver<'chunk>,
{
    value: &'de Value<'chunk, 'resolver>,
    constants: &'cr CR,
}

impl<'de, 'chunk: 'de, 'resolver: 'de, 'cr: 'de, CR>
    ValueDeserializer<'de, 'chunk, 'resolver, 'cr, CR>
where
    CR: ConstantResolver<'chunk>,
{
    /// Construct an instance from a [Value].
    pub fn new(value: &'de Value<'chunk, 'resolver>, constants: &'cr CR) -> Self {
        Self { value, constants }
    }
}

impl<'de, 'chunk: 'de, 'resolver: 'de, 'cr: 'de + 'resolver, CR> Deserializer<'de>
    for ValueDeserializer<'de, 'chunk, 'resolver, 'cr, CR>
where
    CR: ConstantResolver<'chunk>,
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
                    ConstantValueMapped::Null => visitor.visit_none(),
                    ConstantValueMapped::Value(s) => visitor.visit_string(s?),
                    ConstantValueMapped::Missing => visitor.visit_none(),
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
                ConstantValue::Null | ConstantValue::Missing => visitor.visit_none(),
                ConstantValue::Value(v) => {
                    let de = ValueDeserializer {
                        value: v,
                        constants: self.constants,
                    };
                    Self::deserialize_any(de, visitor)
                }
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
                ConstantValue::Null | ConstantValue::Missing => visitor.visit_none(),
                ConstantValue::Value(v) => {
                    let de = ValueDeserializer {
                        value: v,
                        constants: self.constants,
                    };
                    visitor.visit_some(de)
                }
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
pub struct EventsEnumDeserializer<'de, 'chunk: 'de, 'resolver: 'de, CR>
where
    CR: ConstantResolver<'chunk>,
{
    value: &'de Value<'chunk, 'resolver>,
    constants: &'de CR,
}

impl<'de, 'chunk: 'de, 'resolver: 'de, CR> EventsEnumDeserializer<'de, 'chunk, 'resolver, CR>
where
    CR: ConstantResolver<'chunk>,
{
    /// Construct a new instance from a [Value] and [ConstantResolver].
    pub fn new(value: &'de Value<'chunk, 'resolver>, constants: &'de CR) -> Self {
        Self { value, constants }
    }
}

impl<'de, 'chunk: 'de, 'resolver: 'de, CR> EnumAccess<'de>
    for EventsEnumDeserializer<'de, 'chunk, 'resolver, CR>
where
    CR: ConstantResolver<'chunk>,
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

impl<'de, 'chunk: 'de, 'resolver: 'de, CR> VariantAccess<'de>
    for EventsEnumDeserializer<'de, 'chunk, 'resolver, CR>
where
    CR: ConstantResolver<'chunk>,
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

impl<'de, 'chunk: 'de, 'resolver: 'de, CR> Deserializer<'de>
    for EventsEnumDeserializer<'de, 'chunk, 'resolver, CR>
where
    CR: ConstantResolver<'chunk>,
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
