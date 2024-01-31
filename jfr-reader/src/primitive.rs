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
pub enum Primitive<'chunk> {
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
    String(Cow<'chunk, str>),
    /// A string whose value is stored in the constants pool under java.lang.String.
    StringConstantPool(i64),
}

impl<'chunk> Default for Primitive<'chunk> {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl<'chunk> From<bool> for Primitive<'chunk> {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl<'chunk> From<i8> for Primitive<'chunk> {
    fn from(v: i8) -> Self {
        Self::Byte(v)
    }
}

impl<'chunk> From<i16> for Primitive<'chunk> {
    fn from(v: i16) -> Self {
        Self::Short(v)
    }
}

impl<'chunk> From<i32> for Primitive<'chunk> {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl<'chunk> From<i64> for Primitive<'chunk> {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl<'chunk> From<f32> for Primitive<'chunk> {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl<'chunk> From<f64> for Primitive<'chunk> {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl<'chunk> Primitive<'chunk> {
    pub fn parse_boolean(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_boolean(s)?;
        Ok((s, Self::Boolean(v)))
    }

    pub fn parse_char(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_char(s)?;
        Ok((s, Self::Character(v)))
    }

    pub fn parse_float(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_float(s)?;
        Ok((s, Self::Float(v)))
    }

    pub fn parse_double(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_double(s)?;
        Ok((s, Self::Double(v)))
    }

    pub fn parse_byte(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_byte(s)?;
        Ok((s, Self::Byte(v)))
    }

    pub fn parse_short(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_short(s)?;
        Ok((s, Self::Short(v)))
    }

    pub fn parse_int(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_int(s)?;
        Ok((s, Self::Integer(v)))
    }

    pub fn parse_long(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_long(s)?;
        Ok((s, Self::Long(v)))
    }

    pub fn parse_string(s: &'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>> {
        let (s, v) = parse_java_lang_string(s)?;

        let v = match v {
            StringValue::Null => Self::NullString,
            StringValue::String(s) => Self::String(s),
            StringValue::ConstantPoolRef(index) => Self::StringConstantPool(index),
        };

        Ok((s, v))
    }

    /// Resolve a parser function for a primitive value, if available.
    pub fn resolve_parser(
        name: &str,
    ) -> Option<fn(&'chunk [u8]) -> ParseResult<'chunk, Primitive<'chunk>>> {
        match name {
            "boolean" => Some(Self::parse_boolean),
            "char" => Some(Self::parse_char),
            "float" => Some(Self::parse_float),
            "double" => Some(Self::parse_double),
            "byte" => Some(Self::parse_byte),
            "short" => Some(Self::parse_short),
            "int" => Some(Self::parse_int),
            "long" => Some(Self::parse_long),
            "java.lang.String" => Some(Self::parse_string),
            _ => None,
        }
    }

    /// Attempt to parse data as a primitive having the specified class name.
    ///
    /// If the name is not a known primitive, no data should be read.
    pub fn try_parse_from_name(
        name: &str,
        s: &'chunk [u8],
    ) -> ParseResult<'chunk, Option<Primitive<'chunk>>> {
        if let Some(parser) = Self::resolve_parser(name) {
            parser(s).map(|(s, v)| (s, Some(v)))
        } else {
            Ok((s, None))
        }
    }

    /// Obtain the boolean value if a boolean is stored.
    pub fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_byte(&self) -> Option<i8> {
        if let Self::Byte(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_short(&self) -> Option<i16> {
        if let Self::Short(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_integer(&self) -> Option<i32> {
        if let Self::Integer(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_long(&self) -> Option<i64> {
        if let Self::Long(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        if let Self::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        if let Self::Double(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_char(&self) -> Option<char> {
        if let Self::Character(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Obtain the string representation if a resolved string is stored.
    ///
    /// Only returns Some if we are storing a string value. Null strings and
    /// strings in the constant pool return None.
    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(v) = self {
            Some(v.as_ref())
        } else {
            None
        }
    }
}
