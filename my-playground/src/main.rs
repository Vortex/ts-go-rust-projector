use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

fn error_me(throw: bool) -> Result<()> {
    if throw {
        return Err(anyhow!("this should never be true"));
    }

    std::fs::read(PathBuf::from("/foo")).context("Hey unable to do this!");

    return Ok(());
}

fn main() -> Result<()> {
    let value = error_me(false)?;

    println!("Error value: {:?}", value);

    return Ok(());
}
