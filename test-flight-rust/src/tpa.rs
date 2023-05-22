use std::iter::{repeat, zip};
use std::ops::Add;
use std::str::FromStr;
use anyhow::{anyhow, bail, Result};
use crate::actions::Action;
use crate::alpha::{Alpha, ALPHABET};

#[derive(Clone)]
struct TpaGroup([Alpha; 16]);

impl TpaGroup {
    fn new(group: [Alpha; 16]) -> Self {
        Self(group)
    }

    pub fn transform(&self, action_group: Alpha) -> Result<Self> {
        let action = Action::new(action_group).ok_or_else(|| anyhow!("Not a valid action {}", action_group.as_char()))?;
        Ok(TpaGroup(action.transform(self.0)))
    }
}

impl FromStr for TpaGroup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() > 16 {
            bail!("Invalid length {}", s.len())
        }
        Ok(TpaGroup::new(s.chars()
            .map(|c| Alpha::new(c).ok_or_else(|| anyhow!("Invalid uppercase letter '{}'", c)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .chain(ALPHABET.iter().cloned().take(16 - s.len()))
            .collect::<Vec<_>>()
            .try_into().unwrap())
        )
    }
}

impl ToString for TpaGroup {
    fn to_string(&self) -> String {
        self.0.iter()
            .map(Alpha::as_char)
            .collect()
    }
}

impl Add<&TpaGroup> for &TpaGroup {
    type Output = TpaGroup;

    fn add(self, rhs: &TpaGroup) -> Self::Output {
        let alphas = zip(self.0.iter(), rhs.0.iter())
            .map(|(a, b)| a + b )
            .collect::<Vec<_>>()
            .try_into().unwrap();
        TpaGroup(alphas)
    }
}

pub fn marvin_tpa(seed: impl AsRef<str>) -> Result<String> {
    let seed = seed.as_ref().replace(char::is_whitespace, "");
    let num_groups = (seed.len() + 15) / 16;
    let seed: String = repeat('A').take(num_groups).chain(seed.chars())
        .collect();
    marvin_tpa_12(seed)
}

pub fn marvin_tpa_12(seed: impl AsRef<str>) -> Result<String> {
    let seed = seed.as_ref().replace(char::is_whitespace, "");
    // Preamble length is equal to the number of groups if each group was 17 characters long
    let preamble_len = (seed.len() + 16) / 17;
    let preamble = seed[..preamble_len].chars()
        .map(|c| Alpha::new(c).ok_or_else(|| anyhow!("Not an uppercase letter {}", c)))
        .collect::<Result<Vec<_>>>()?;
    let seed = &seed[preamble_len..];
    let reduction = (0..seed.len()).step_by(16).zip(preamble.into_iter()).map(|(start, action_group)| {
        let end = (start + 16).min(seed.len());
        let slice = &seed[start..end];
        let group = TpaGroup::from_str(slice)?;
        group.transform(action_group)
    })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .reduce(|a, b| &a + &b)
        .unwrap();
    Ok(reduction.to_string())
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::alpha::Alpha;
    use super::{marvin_tpa, marvin_tpa_12, TpaGroup};

    #[test]
    fn test_marvin_tpa() {
        // Given a seed
        let seed = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        // When the seed is reduced
        let reduction = marvin_tpa(seed).unwrap();

        // The it is reduced as expected
        assert_eq!("TTTNANHHHCZCXTGT", reduction);
    }

    #[test]
    fn test_marvin_tpa_12() {
        // Given a seed
        let seed = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        // When the seed is reduced
        let reduction = marvin_tpa_12(seed).unwrap();

        // The it is reduced as expected
        assert_eq!("HTVUNWOZVUNXZAPB", reduction);
    }

    #[test]
    fn test_tpa_group_reduce() {
        // Given a string and it's expected transformation
        for (group, action, expected) in [
            ("ABCDEFGHIJKLMNOP", 'A', "OACYZXIUWTESQOAP"),
            ("QRSTUVWXYZ",       'A', "ESQOAPYMKIUJGEFD"),
            ("ABCDEFGHIJKLMNOP", 'B', "SFWJANERCPGTKXOB"),
            ("ABCDEFGHIJKLMNOP", 'C', "PPGGXOOXFFWAAWRR"),
            ("ABCDEFGHIJKLMNOP", 'D', "OOKKGGCCEEAAWWSS"),
            ("ABCDEFGHIJKLMNOP", 'E', "CNBMYKWHVGSDRCOA"),
            ("ABCDEFGHIJKLMNOP", 'F', "OAPEDFUITJYXAOZP"),
        ] {
            // and a TPA Group corresponding to the string
            let group = TpaGroup::from_str(group).unwrap();

            // When the group is transformed
            let reduction = group.transform(Alpha::new(action).unwrap()).unwrap();

            // Then the result is as expected
            assert_eq!(expected, reduction.to_string())
        }
    }
}
