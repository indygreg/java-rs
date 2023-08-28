// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Handling of JFR value types.

use {
    crate::{
        common::{leb128_i16, leb128_i32, leb128_i64},
        error::{NomParseError, ParseResult},
        string_table::{StringRecord, StringValue},
    },
    nom::number::streaming::{be_f32, be_f64, be_i8},
    std::borrow::Cow,
};

pub fn parse_boolean(s: &[u8]) -> ParseResult<bool> {
    let (s, v) = be_i8(s)?;
    Ok((s, v != 0))
}

pub fn parse_float(s: &[u8]) -> ParseResult<f32> {
    let (s, v) = be_f32(s)?;
    Ok((s, v))
}

pub fn parse_double(s: &[u8]) -> ParseResult<f64> {
    let (s, v) = be_f64(s)?;
    Ok((s, v))
}

pub fn parse_byte(s: &[u8]) -> ParseResult<i8> {
    let (s, v) = be_i8(s)?;
    Ok((s, v))
}

pub fn parse_short(s: &[u8]) -> ParseResult<i16> {
    let (s, v) = leb128_i16(s)?;
    Ok((s, v))
}

pub fn parse_int(s: &[u8]) -> ParseResult<i32> {
    let (s, v) = leb128_i32(s)?;
    Ok((s, v))
}

pub fn parse_long(s: &[u8]) -> ParseResult<i64> {
    let (s, v) = leb128_i64(s)?;
    Ok((s, v))
}

pub fn parse_java_lang_string(s: &[u8]) -> ParseResult<StringValue> {
    let (s, record) = StringRecord::parse(s)?;
    let (_, v) = record.resolve()?;

    Ok((s, v))
}

pub fn parse_char(s: &[u8]) -> ParseResult<char> {
    let (s, v) = leb128_i32(s)?;

    let v = char::try_from(v as u32)
        .map_err(|e| nom::Err::Failure(NomParseError::new_string_resolve(s, e.into())))?;

    Ok((s, v))
}

/// A Java primitive value.
#[derive(Clone, Debug)]
pub enum Primitive<'a> {
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Character(char),
    /// The null string.
    NullString,
    /// A string with resolved value.
    String(Cow<'a, str>),
    /// A string whose value is stored in the constants pool under java.lang.String.
    StringConstantPool(i64),
}

impl<'a> Default for Primitive<'a> {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl<'a> From<bool> for Primitive<'a> {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl<'a> From<i8> for Primitive<'a> {
    fn from(v: i8) -> Self {
        Self::Byte(v)
    }
}

impl<'a> From<i16> for Primitive<'a> {
    fn from(v: i16) -> Self {
        Self::Short(v)
    }
}

impl<'a> From<i32> for Primitive<'a> {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl<'a> From<i64> for Primitive<'a> {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl<'a> From<f32> for Primitive<'a> {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl<'a> From<f64> for Primitive<'a> {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl<'a> Primitive<'a> {
    pub fn try_parse_from_name(name: &str, s: &'a [u8]) -> ParseResult<'a, Option<Primitive<'a>>> {
        match name {
            // This seems to be the order of the constant class IDs that are
            // emitted. But order doesn't matter and we make no assumptions
            // about the class IDs.
            "boolean" => {
                let (s, v) = parse_boolean(s)?;
                Ok((s, Some(Self::Boolean(v))))
            }
            "char" => {
                let (s, v) = parse_char(s)?;
                Ok((s, Some(Self::Character(v))))
            }
            "float" => {
                let (s, v) = parse_float(s)?;
                Ok((s, Some(Self::Float(v))))
            }
            "double" => {
                let (s, v) = parse_double(s)?;
                Ok((s, Some(Self::Double(v))))
            }
            "byte" => {
                let (s, v) = parse_byte(s)?;
                Ok((s, Some(Self::Byte(v))))
            }
            "short" => {
                let (s, v) = parse_short(s)?;
                Ok((s, Some(Self::Short(v))))
            }
            "int" => {
                let (s, v) = parse_int(s)?;
                Ok((s, Some(Self::Integer(v))))
            }
            "long" => {
                let (s, v) = parse_long(s)?;
                Ok((s, Some(Self::Long(v))))
            }
            "java.lang.String" => {
                let (s, v) = parse_java_lang_string(s)?;

                let v = Some(match v {
                    StringValue::Null => Self::NullString,
                    StringValue::String(s) => Self::String(s),
                    StringValue::ConstantPoolRef(index) => Self::StringConstantPool(index),
                });

                Ok((s, v))
            }
            _ => Ok((s, None)),
        }
    }
}
