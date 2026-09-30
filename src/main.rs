#![doc = include_str!("../README.md")]

use anyhow::Result;
use seed::{init_logger, print_header};

fn main() -> Result<()> {
    print_header();
    init_logger()?;
    Ok(())
}
