// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use jfr_metadata_xml::Metadata;
use jfr_reader::metadata_xml::{metadata_to_rs, Result};

fn main() -> Result<()> {
    let args = std::env::args_os().collect::<Vec<_>>();

    if args.len() < 2 {
        println!(
            "Usage: {} path/to/metadata.xml",
            std::env::current_exe().unwrap().display()
        );
        std::process::exit(1);
    }

    let input = std::path::PathBuf::from(&args[1]);

    let m = Metadata::from_path(input)?;

    println!("{}", metadata_to_rs(&m)?);

    Ok(())
}
