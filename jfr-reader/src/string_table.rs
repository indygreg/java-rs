// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR String Table and string values.
//!
//! JFR strings are expressed via a simple header denoting the string
//! encoding and optional inline content defining the string data or where
//! the string data is found.
//!
//! String encoding is represented by the [Encoding] enum. The common
//! serialized string record by [StringRecordHeader] and [StringRecord].
//!
//! The [StringValue] enum denotes a resolved/decoded string.
//!
//! The [LazyStringTable] struct provides an interface for the string
//! table (located in the metadata event). As its name implies, it
//! supports lazily decoding strings upon first access.

use {
    crate::{
        common::{leb128_i32, leb128_i64},
        error::{Error, NomParseError, ParseResult, Result, StringResolveError},
    },
    nom::{bytes::streaming::take, multi::count, number::complete::be_u8},
    num_enum::TryFromPrimitive,
    std::borrow::Cow,
};

/// Represents the byte enumeration of the encoding of a string.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Encoding {
    Null,
    EmptyString,
    ConstantPool,
    Utf8ByteArray,
    CharArray,
    Latin1ByteArray,
}

/// Represents the static sized data in the header of a string table entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringRecordHeader {
    /// The null entry. No data.
    Null,

    /// Represents the empty string. ""
    Empty,

    /// A reference to an entry in a constants pool.
    ConstantPool(i64),

    /// A UTF-8 string.
    Utf8ByteArray(i32),

    /// A character array.
    ///
    /// Logically an array of Java char. Value is number of characters,
    /// which can be variable width because of LEB-128 encoding.
    CharArray(i32),

    /// A string composed of Latin-1 encoded characters.
    Latin1ByteArray(i32),
}

impl StringRecordHeader {
    /// Parse a string record header.
    ///
    /// Records can be as short as 1 byte and as long as 9 bytes.
    ///
    /// Any inline string data is not read. If inline string data is present,
    /// it will follow this decoded record.
    pub fn parse(s: &[u8]) -> ParseResult<Self> {
        let (s, encoding) = be_u8(s)?;

        let encoding = Encoding::try_from(encoding).map_err(|_| {
            nom::Err::Failure(NomParseError::new_string_resolve(
                &[encoding],
                StringResolveError::UnknownStringEncoding(encoding),
            ))
        })?;

        let (s, res) = match encoding {
            Encoding::Null => (s, Self::Null),
            Encoding::EmptyString => (s, Self::Empty),
            Encoding::ConstantPool => {
                let (s, v) = leb128_i64(s)?;

                (s, Self::ConstantPool(v))
            }
            Encoding::Utf8ByteArray => {
                let (s, size) = leb128_i32(s)?;

                (s, Self::Utf8ByteArray(size))
            }
            Encoding::CharArray => {
                let (s, size) = leb128_i32(s)?;

                (s, Self::CharArray(size))
            }
            Encoding::Latin1ByteArray => {
                let (s, size) = leb128_i32(s)?;

                (s, Self::Latin1ByteArray(size))
            }
        };

        Ok((s, res))
    }
}

/// Represents a record in a string table with a full reference to inline string data.
///
/// The inline string data is not decoded. So instances should be relatively
/// cheap to create.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StringRecord<'a> {
    /// The null entry. No data.
    Null,

    /// Represents the empty string. ""
    Empty,

    /// A reference to an entry in a constants pool.
    ConstantPool(i64),

    /// A UTF-8 string.
    Utf8ByteArray(&'a [u8]),

    /// A character array.
    ///
    /// String is composed of bytes.
    CharArray(Vec<i32>),

    /// A string composed of Latin-1 encoded characters.
    Latin1ByteArray(&'a [u8]),
}

impl<'a> StringRecord<'a> {
    /// Parse a string record.
    ///
    /// Records can be as short as 1 byte and as long as 2 + 2^31-1 bytes.
    ///
    /// Callers are advised to pass in a slice at least of length 9 to guarantee
    /// they are able to decode the static header and at least know how long any
    /// inline string data is. Assuming this to be true, any errors should indicate
    /// how many remaining bytes of data need to be acquired to obtain a reference
    /// to inline string data.
    pub fn parse(s: &'a [u8]) -> ParseResult<Self> {
        let (s, header) = StringRecordHeader::parse(s)?;

        let (s, res) = match header {
            StringRecordHeader::Null => (s, Self::Null),
            StringRecordHeader::Empty => (s, Self::Empty),
            StringRecordHeader::ConstantPool(v) => (s, Self::ConstantPool(v)),
            StringRecordHeader::Utf8ByteArray(size) => {
                let (s, data) = take(size as usize)(s)?;

                (s, Self::Utf8ByteArray(data))
            }
            StringRecordHeader::CharArray(size) => {
                let (s, data) = count(leb128_i32, size as _)(s)?;

                (s, Self::CharArray(data))
            }
            StringRecordHeader::Latin1ByteArray(size) => {
                let (s, data) = take(size as usize)(s)?;

                (s, Self::Latin1ByteArray(data))
            }
        };

        Ok((s, res))
    }

    /// Attempt to coerce this record to a string.
    ///
    /// The decoded string always has a static lifetime.
    pub fn resolve(&self) -> ParseResult<StringValue<'static>> {
        match self {
            Self::Null => Ok((&[], StringValue::Null)),
            Self::Empty => Ok((&[], StringValue::String(Cow::Borrowed("")))),
            StringRecord::ConstantPool(index) => Ok((&[], StringValue::ConstantPoolRef(*index))),
            StringRecord::Utf8ByteArray(raw) => {
                let v = StringValue::String(Cow::Owned(String::from_utf8(raw.to_vec()).map_err(
                    |e| nom::Err::Failure(NomParseError::new_string_resolve(raw, e.into())),
                )?));

                Ok((raw, v))
            }
            StringRecord::CharArray(raw) => {
                match raw
                    .iter()
                    .map(|x| char::try_from(*x as u32))
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(chars) => Ok((
                        &[],
                        StringValue::String(Cow::Owned(String::from_iter(chars.into_iter()))),
                    )),
                    Err(err) => {
                        let mut bytes = Vec::with_capacity(4 * raw.len());
                        bytes.extend(raw.iter().flat_map(|x| x.to_ne_bytes().into_iter()));

                        Err(nom::Err::Failure(NomParseError::new_string_resolve(
                            &bytes,
                            err.into(),
                        )))
                    }
                }
            }
            StringRecord::Latin1ByteArray(raw) => {
                // Map each u8 to a char.
                let res = String::from_iter(raw.iter().map(|x| *x as char));

                Ok((raw, StringValue::String(Cow::Owned(res))))
            }
        }
    }
}

/// Represents a decoded string value.
pub enum StringValue<'a> {
    /// The null string.
    Null,
    /// String is stored in the constants pool for java.lang.String at this index.
    ConstantPoolRef(i64),
    /// The decoded string content.
    String(Cow<'a, str>),
}

impl<'a> StringValue<'a> {
    /// Obtain the str representation of self if available.
    ///
    /// Null strings and constant pool references return None.
    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(v) = self {
            Some(v.as_ref())
        } else {
            None
        }
    }

    pub fn as_cow(&self) -> Option<Cow<'a, str>> {
        if let Self::String(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }
}

enum LazilyDecodedString<'a> {
    Unparsed(StringRecord<'a>),
    Parsed(Result<StringValue<'static>>),
}

struct LazilyDecodedRecord<'a>(LazilyDecodedString<'a>);

impl<'a> LazilyDecodedRecord<'a> {
    fn ensure_resolved(&mut self) {
        let res = match &self.0 {
            LazilyDecodedString::Parsed(_) => {
                return;
            }
            LazilyDecodedString::Unparsed(sr) => sr.resolve(),
        };

        self.0 = LazilyDecodedString::Parsed(match res {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        });
    }
}

/// A string table that lazily converts memory slices to string types.
pub struct LazyStringTable<'a> {
    entries: Vec<LazilyDecodedRecord<'a>>,
}

impl<'a> From<Vec<StringRecord<'a>>> for LazyStringTable<'a> {
    fn from(records: Vec<StringRecord<'a>>) -> Self {
        Self {
            entries: records
                .into_iter()
                .map(|x| LazilyDecodedRecord(LazilyDecodedString::Unparsed(x)))
                .collect::<Vec<_>>(),
        }
    }
}

impl<'a> LazyStringTable<'a> {
    /// The count of entries in this table.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether this table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Obtain an entry from the string table, triggering string decoding as necessary.
    ///
    /// The result of string decoding is cached on the instance. Subsequent
    /// lookups of this key will return a reference to an already decoded string
    /// or an error for its failure to decode.
    pub fn get(&mut self, index: usize) -> Result<&StringValue<'a>> {
        match self.entries.get_mut(index) {
            None => Err(Error::StringTableUnknownIndex(index)),
            Some(record) => {
                record.ensure_resolved();

                match &record.0 {
                    LazilyDecodedString::Parsed(Ok(v)) => Ok(v),
                    LazilyDecodedString::Parsed(Err(e)) => Err(e.clone()),
                    LazilyDecodedString::Unparsed(_) => {
                        panic!("how did this happen?")
                    }
                }
            }
        }
    }
}
