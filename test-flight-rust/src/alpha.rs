use std::fmt::{Display, Formatter};
use std::ops::Add;
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Alpha(pub u8);

impl Alpha {
    pub const fn new(c: char) -> Option<Self> {
        if c.is_ascii_uppercase() {
            Some(Self(c as u8 - 'A' as u8 + 1))
        } else {
            None
        }
    }

    pub fn is_vowel(&self) -> bool {
        match self.0 {
            1 | 5 | 9 | 15 | 21 | 25 => true,
            _ => false
        }
    }

    pub fn rot13(&self) -> Self {
        Self((self.0 - 1 + 13) % 26 + 1)
    }

    pub fn as_char(&self) -> char {
        (self.0 - 1 + 'A' as u8) as char
    }
}

impl Display for Alpha {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl From<Alpha> for char {
    fn from(value: Alpha) -> Self {
        value.as_char()
    }
}

impl Add<&Alpha> for &Alpha {
    type Output = Alpha;

    fn add(self, rhs: &Alpha) -> Self::Output {
        Alpha((self.0 + rhs.0 - 1) % 26 + 1)
    }
}

pub static ALPHABET: Lazy<[Alpha; 26]> = Lazy::new(|| {
    ('A'..='Z').into_iter()
        .map(|c| Alpha::new(c).unwrap())
        .collect::<Vec<_>>()
        .try_into().unwrap()
});
