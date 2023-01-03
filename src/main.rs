#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use crate::prelude::*;

mod error;
mod prelude;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
