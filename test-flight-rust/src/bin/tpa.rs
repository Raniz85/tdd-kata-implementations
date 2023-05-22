use std::fmt::{Display, Formatter};
use std::io::{Read, stdin};
use std::iter::{repeat, zip};
use std::ops::Add;
use std::str::FromStr;
use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;

use crate::alpha::{Alpha, ALPHABET};
use crate::actions::Action;

fn main() -> Result<()> {
    println!("Input seed:");
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let seed: String = input.lines()
        .map(str::trim)
        .collect();
    println!("{}", marvin_tpa_12(seed)?);
    Ok(())
}