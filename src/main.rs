#![doc = include_str!("../README.md")]

use anyhow::Result;
use seed::{init_logger, print_header};

fn main() -> Result<()> {
    init_logger()?;
    print_header();
    Ok(())
}
