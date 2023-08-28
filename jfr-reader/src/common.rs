// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Common functionality.

use crate::error::ParseResult;
use nom::number::streaming::be_i8;

/// Read an LEB-128 encoded integer.
pub fn leb128_i64(mut s: &[u8]) -> ParseResult<i64> {
    let mut res = 0;

    let mut x: i8;

    for i in 0..8 {
        (s, x) = be_i8(s)?;

        res += (x as i64 & 0x7f) << (7 * i) as i64;
        if x >= 0 {
            return Ok((s, res));
        }
    }

    let (s, x) = be_i8(s)?;
    res += (x as i64 & 0xff) << 56;

    Ok((s, res))
}

pub fn leb128_i16(s: &[u8]) -> ParseResult<i16> {
    let (s, x) = leb128_i64(s)?;

    Ok((s, x as i16))
}

pub fn leb128_i32(s: &[u8]) -> ParseResult<i32> {
    let (s, x) = leb128_i64(s)?;

    Ok((s, x as i32))
}
