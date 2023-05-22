use std::fmt::{Display, Formatter};
use std::io::{Read, stdin};
use std::iter::{repeat, zip};
use std::ops::Add;
use std::str::FromStr;
use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;

pub mod actions;
pub mod alpha;
pub mod tpa;
pub mod map;