// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nom::{
    error::{ContextError, ErrorKind, ParseError},
    IResult,
};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum StringResolveError {
    #[error("unknown string encoding: {0}")]
    UnknownStringEncoding(u8),

    #[error("Invalid character array: {0}")]
    InvalidCharacterArray(#[from] std::char::CharTryFromError),

    #[error("Invalid UTF-8 string data")]
    InvalidUtf8String(#[from] std::string::FromUtf8Error),
}

#[derive(Clone, Debug)]
pub struct NomParseError {
    pub input: Vec<u8>,
    pub kind: ErrorKind,
    pub contexts: Vec<&'static str>,
    pub string_resolve: Option<StringResolveError>,
}

impl<'a> ParseError<&'a [u8]> for NomParseError {
    fn from_error_kind(input: &'a [u8], kind: ErrorKind) -> Self {
        Self {
            input: input.to_vec(),
            kind,
            contexts: vec![],
            string_resolve: None,
        }
    }

    fn append(_: &'a [u8], _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> ContextError<&'a [u8]> for NomParseError {
    fn add_context(_input: &'a [u8], ctx: &'static str, mut other: Self) -> Self {
        other.contexts.push(ctx);

        other
    }
}

impl<'a> NomParseError {
    /// Construct an instance from a [StringResolveError].
    pub fn new_string_resolve(input: &'a [u8], error: StringResolveError) -> Self {
        Self {
            input: input.to_vec(),
            kind: ErrorKind::Fail,
            contexts: vec![],
            string_resolve: Some(error),
        }
    }
}

pub type ParseResult<'a, T> = IResult<&'a [u8], T, NomParseError>;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("insufficient input data for parsing: {0:?}")]
    ParseIncomplete(nom::Needed),

    #[error("parse error: {0:?}")]
    ParseError(NomParseError),

    #[error("parse failure: {0:?}")]
    ParseFailure(NomParseError),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("unknown index into string table: {0}")]
    StringTableUnknownIndex(usize),

    #[error("unable to resolve a name for a metadata element")]
    ElementNoName,

    #[error("element name unknown: {0}")]
    ElementNameUnknown(String),

    #[error("logic error when constructing elements: {0}")]
    ElementConstructLogic(String),

    #[error("failed to locate class with id {0}")]
    ClassNotFound(i64),

    #[error("could not find constant {1} for class {0}")]
    ConstantNotFound(i64, i64),

    #[error("event parsing: {0}")]
    EventParse(String),

    #[error("annotation parsing: {0}")]
    AnnotationParse(String),

    #[error("setting parsing: {0}")]
    SettingParse(String),
}

impl From<nom::Err<NomParseError>> for Error {
    fn from(value: nom::Err<NomParseError>) -> Self {
        match value {
            nom::Err::Incomplete(needed) => Self::ParseIncomplete(needed),
            nom::Err::Error(e) => Self::ParseError(e),
            nom::Err::Failure(e) => Self::ParseFailure(e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
