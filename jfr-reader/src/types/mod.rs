// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Defines high-level event types and referenced data structures.

#[cfg(feature = "openjdk17")]
pub mod openjdk17;
#[cfg(feature = "openjdk21")]
pub mod openjdk21;
