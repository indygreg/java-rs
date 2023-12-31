// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Constant pool primitives.

use {
    crate::{
        common::{leb128_i32, leb128_i64},
        error::{ParseResult, Result},
        resolver::EventResolver,
        value::Value,
    },
    nom::{bytes::streaming::take, error::context, number::streaming::be_i8, sequence::pair},
};

/// The full header of a constants pool event record.
///
/// Includes the size and class ID common event header fields.
#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantPoolHeader {
    pub size: i32,
    pub type_id: i64,
    pub timestamp: i64,
    pub duration: i64,
    pub delta: i64,
    pub mask: i8,
    pub pool_count: i32,
}

impl ConstantPoolHeader {
    pub fn parse(s: &[u8]) -> ParseResult<Self> {
        let (s, size) = leb128_i32(s)?;
        // Should be constant pool type id.
        let (s, type_id) = leb128_i64(s)?;

        let (s, timestamp) = leb128_i64(s)?;
        let (s, duration) = leb128_i64(s)?;
        let (s, delta) = leb128_i64(s)?;
        let (s, mask) = be_i8(s)?;
        let (s, pool_count) = leb128_i32(s)?;

        Ok((
            s,
            Self {
                size,
                type_id,
                timestamp,
                duration,
                delta,
                mask,
                pool_count,
            },
        ))
    }
}

fn parse_constant_pool_value<'a, 'r>(
    s: &'a [u8],
    resolver: &'r EventResolver<'a>,
    class_id: i64,
) -> Result<(&'a [u8], i64, Value<'r>)> {
    let (s, pool_index) = leb128_i64(s)?;

    // Constant pool values can resolve to primitives (notably strings). So
    // we need to resolve Value here and not Object.
    let (s, value) = resolver.parse_value(s, class_id)?;

    Ok((s, pool_index, value))
}

fn parse_constant_pool_class<'a, 'r>(
    s: &'a [u8],
    resolver: &'r EventResolver<'a>,
) -> Result<(&'a [u8], i64, Vec<(i64, Value<'r>)>)> {
    let (mut s, (class_id, constant_count)) = context(
        "parsing constant pool class entry",
        pair(leb128_i64, leb128_i32),
    )(s)?;

    let mut res = Vec::with_capacity(constant_count as usize);

    for _ in 0..constant_count {
        let (remaining, index, value) = parse_constant_pool_value(s, resolver, class_id)?;
        res.push((index, value));
        s = remaining;
    }

    Ok((s, class_id, res))
}

/// Holds a parsed constants pool header and a reference to its data.
#[derive(Clone, Debug)]
pub struct ConstantPoolEvent<'a> {
    pub header: ConstantPoolHeader,
    /// Holds constants pool data.
    ///
    /// Not inclusive of header.
    pub pool_data: &'a [u8],
}

impl<'a> ConstantPoolEvent<'a> {
    pub fn parse(s: &'a [u8]) -> ParseResult<Self> {
        let (pool_data, header) =
            context("parsing constant pool header", ConstantPoolHeader::parse)(s)?;

        let (s, _) = context(
            "reading constant pool event data",
            take(header.size as usize),
        )(s)?;

        Ok((s, Self { header, pool_data }))
    }

    /// Iterate over constants in this constant pool.
    ///
    /// Each entry is composed of its class/type ID, its index value in the constants pool,
    /// and its parsed value.
    ///
    /// Sadly it isn't possible to lazily parse constants pool entries into
    /// values because the entries do not encode their own size. We need to
    /// decode each entry in full using the chunk's typing metadata in order
    /// to identify boundaries between constants in the pool.
    pub fn resolve_constants<'r>(
        &self,
        resolver: &'r EventResolver<'a>,
    ) -> Result<Vec<(i64, Vec<(i64, Value<'r>)>)>> {
        let mut s = self.pool_data;

        let mut res = Vec::new();

        for _ in 0..self.header.pool_count {
            let (remaining, class_id, values) = parse_constant_pool_class(s, resolver)?;
            s = remaining;

            res.push((class_id, values));
        }

        Ok(res)
    }
}
