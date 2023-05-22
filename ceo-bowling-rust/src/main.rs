use anyhow::{anyhow, bail, Error, Result};
use itertools::{Itertools, process_results};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let variant: Box<dyn ScoreCalculator> = match args.next() {
        Some(variant) if variant.eq_ignore_ascii_case("variant1") || variant.eq_ignore_ascii_case("1") => Box::new(Variant1::default()),
        Some(variant) if variant.eq_ignore_ascii_case("variant2") || variant.eq_ignore_ascii_case("2") => Box::new(Variant2::default()),
        Some(variant) if variant.eq_ignore_ascii_case("variant3") || variant.eq_ignore_ascii_case("3") => Box::new(Variant3::default()),
        Some(variant) if variant.eq_ignore_ascii_case("variant4") || variant.eq_ignore_ascii_case("4") => Box::new(Variant4::default()),
        Some(variant) if variant.eq_ignore_ascii_case("variant5") || variant.eq_ignore_ascii_case("5") => Box::new(Variant5::default()),
        Some(variant) => bail!("Invalid scoring variant {}", variant),
        None => Box::new(Variant1::default()),
    };
    let input_files = args.collect_vec();
    if input_files.is_empty() {
        bail!("No input");
    }
    let scorecards = input_files.iter().map(|input_file| {
        let mut input = String::new();
        File::open(input_file).and_then(|mut f| f.read_to_string(&mut input))?;
        Ok(input)
    }).collect::<Result<Vec<_>, Error>>()?;
    let winner = get_winner(&scorecards, variant.as_ref())?;
    println!("The winner is {} with a score of {}", winner.0, winner.1);
    Ok(())
}

enum Frame {
    Regular(u8, u8),
    Spare(u8),
    Strike,
}

trait ScoreCalculator {
    fn calculate_score(&self, series: &[Frame]) -> u32;
}

#[derive(Default)]
struct Variant1;

impl ScoreCalculator for Variant1 {
    fn calculate_score(&self, series: &[Frame]) -> u32 {
        series.iter()
            .map(|roll| match roll {
                Frame::Regular(first, second) => (first + second) as u32,
                Frame::Spare(_) | Frame::Strike => 10u32,
            })
            .sum()
    }
}

struct Variant2 {
    spare_bonus: u32,
    strike_bonus: u32,
}

impl Default for Variant2 {
    fn default() -> Self {
        Variant2 {
            spare_bonus: 5,
            strike_bonus: 10,
        }
    }
}

impl ScoreCalculator for Variant2 {
    fn calculate_score(&self, series: &[Frame]) -> u32 {
        series.iter()
            .map(|roll| match roll {
                Frame::Regular(first, second) => (first + second) as u32,
                Frame::Spare(_) => 10 + self.spare_bonus,
                Frame::Strike => 10 + self.strike_bonus,
            })
            .sum()
    }
}

struct Variant3 {
    spare_bonus: u32,
    spare_increment: u32,
    strike_bonus: u32,
    strike_increment: u32,
}

impl Default for Variant3 {
    fn default() -> Self {
        Variant3 {
            spare_bonus: 5,
            spare_increment: 1,
            strike_bonus: 10,
            strike_increment: 2,
        }
    }
}

impl ScoreCalculator for Variant3 {
    fn calculate_score(&self, series: &[Frame]) -> u32 {
        series.iter()
            .fold((0u32, self.spare_bonus, self.strike_bonus), |state, frame| {
                let (score, spare_bonus, strike_bonus) = state;
                let score = score + match frame {
                    Frame::Regular(first, second) => (first + second) as u32,
                    Frame::Spare(_) => 10 + spare_bonus,
                    Frame::Strike => 10 + strike_bonus,
                };
                let (spare_bonus, strike_bonus) = match frame {
                    Frame::Regular(_, _) => (spare_bonus, strike_bonus),
                    Frame::Spare(_) => (spare_bonus + self.spare_increment, strike_bonus),
                    Frame::Strike => (spare_bonus, strike_bonus + self.strike_increment),
                };
                (score, spare_bonus, strike_bonus)
            }).0
    }
}

struct Variant4 {}

impl Default for Variant4 {
    fn default() -> Self {
        Variant4 {
        }
    }
}

impl ScoreCalculator for Variant4 {
    fn calculate_score(&self, series: &[Frame]) -> u32 {
        series.iter()
            .rev()
            .fold((0u32, 0u8, 0u8), |state, frame| {
                let (score, next_roll, second_next_roll) = state;
                let score = score + match frame {
                    Frame::Regular(first, second) => (first + second) as u32,
                    Frame::Spare(_) => (10 + next_roll) as u32,
                    Frame::Strike => (10 + next_roll + second_next_roll) as u32,
                };
                let (next_roll, second_next_roll) = match frame {
                    Frame:: Regular(first, second) => (*first, *second),
                    Frame::Spare(first) => (*first, 10 - first),
                    Frame::Strike => (10, next_roll),
                };
                (score, next_roll, second_next_roll)
            }).0
    }
}
struct Variant5 {
    variant1: Variant1,
    variant2: Variant2,
    variant3: Variant3,
    variant4: Variant4,
}

impl Default for Variant5 {
    fn default() -> Self {
        Variant5 {
            variant1: Variant1::default(),
            variant2: Variant2::default(),
            variant3: Variant3::default(),
            variant4: Variant4::default(),
        }
    }
}

impl ScoreCalculator for Variant5 {
    fn calculate_score(&self, series: &[Frame]) -> u32 {
        let variants: &[&dyn ScoreCalculator] = &[&self.variant1, &self.variant2, &self.variant3, &self.variant4];
            variants.iter()
            .map(|variant| variant.calculate_score(series))
            .sum()
    }
}

fn calculate_score<'a>(line: &'a str, variant: &dyn ScoreCalculator) -> Result<(&'a str, u32)> {
    let Some(score_start) = line.find(char::is_numeric) else {
        return Ok((line.trim(), 0));
    };
    let (name, scores) = line.split_at(score_start);
    let name = name.trim();
    let score = process_results(scores.split(" ").map(u8::from_str), |mut scores| -> Result<u32, Error> {
        let mut series = Vec::new();
        loop {
            let Some(first_roll) = scores.next() else {
                break;
            };
            let roll = if first_roll == 10 {
                Frame::Strike
            } else {
                let second_roll = scores.next().ok_or_else(|| anyhow!("Invalid scorecard"))?;
                if first_roll + second_roll == 10 {
                    Frame::Spare(first_roll)
                } else {
                    Frame::Regular(first_roll, second_roll)
                }
            };
            series.push(roll);
        }
        Ok(variant.calculate_score(&series))
    })??;
    Ok(dbg!((name, score)))
}

fn get_winner<'a>(scorecards: &'a[impl AsRef<str>], variant: &dyn ScoreCalculator) -> Result<(&'a str, u32)> {
    process_results(scorecards.iter()
                        .flat_map(|scorecard|
                            scorecard.as_ref()
                                .split("\n")
                                .map(|series| calculate_score(series, variant))),
                    |scores| scores
                        .sorted_by_key(|p| p.0)
                        .into_grouping_map_by(|p| p.0)
                        .fold(0u32, |total, _, p| total + p.1)
                        .into_iter()
                        .max_by_key(|p| p.1)
                        .ok_or_else(|| anyhow!("No participants in scorecard")))?
}

#[cfg(test)]
mod tests {
    use crate::{calculate_score, get_winner, Variant1, Variant2, Variant3, Variant4, Variant5};

    #[test]
    fn test_calculate_score() {
        for (line, expected_result) in [
            (
                "Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3",
                ("Yattas Del Lana", 45),
            ),
            ("Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0", ("Eve Stojbs", 42)),
        ] {
            let variant = Variant1::default();
            assert_eq!(calculate_score(line, &variant).unwrap(), expected_result);
        }
    }

    #[test]
    fn test_calculate_score_variant2() {
        for (line, expected_result) in [
            (
                "Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3",
                ("Yattas Del Lana", 55),
            ),
            ("Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0", ("Eve Stojbs", 57)),
        ] {
            let variant = Variant2::default();
            assert_eq!(calculate_score(line, &variant).unwrap(), expected_result);
        }
    }

    #[test]
    fn test_calculate_score_variant3() {
        for (line, expected_result) in [
            (
                "Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3",
                ("Yattas Del Lana", 55),
            ),
            ("Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0", ("Eve Stojbs", 60)),
        ] {
            let variant = Variant3::default();
            assert_eq!(calculate_score(line, &variant).unwrap(), expected_result);
        }
    }

    #[test]
    fn test_calculate_score_variant4() {
        for (line, expected_result) in [
            (
                "Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3",
                ("Yattas Del Lana", 52),
            ),
            ("Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0", ("Eve Stojbs", 53)),
        ] {
            let variant = Variant4::default();
            assert_eq!(calculate_score(line, &variant).unwrap(), expected_result);
        }
    }

    #[test]
    fn test_calculate_score_variant5() {
        for (line, expected_result) in [
            (
                "Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3",
                ("Yattas Del Lana", 207),
            ),
            ("Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0", ("Eve Stojbs", 212)),
        ] {
            let variant = Variant5::default();
            assert_eq!(calculate_score(line, &variant).unwrap(), expected_result);
        }
    }

    #[test]
    fn test_get_winner() {
        // Given a scorecard and an aexpected winner
        for (input, expected_winner) in [
            ("Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3\nEve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0\n", ("Yattas Del Lana", 45)),
            ("Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3\nEve Stojbs 3 7 3 3 9 1 6 4 2 3 1 5\n", ("Eve Stojbs", 47)),
        ] {
            // And scoring variant 1
            let variant = Variant1::default();

            // Expect the winner to be as expected
            assert_eq!(get_winner(&[input], &variant).unwrap(), expected_winner)
        }
    }

    #[test]
    fn test_get_winner_variant2() {
        for input in [
            "\
            Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3\n\
            Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0\n\
            ",
            "\
            Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0\n\
            Yattas Del Lana 1 5 3 2 7 3 3 0 10 4 3\n\
            ",
        ] {
            let variant = Variant2::default();
            assert_eq!(get_winner(&[input], &variant).unwrap(), ("Eve Stojbs", 57))
        }
    }

    #[test]
    fn test_get_winner_multiple_scorecards() {
        let input = [
            "\
            Yattas Del Lana 3 5 3 5 7 2 3 0 10 4 3\n\
            Eve Stojbs 3 7 3 3 9 1 6 4 2 3 1 0\n\
            ",
            "\
            Eve Stojbs 1 1\n\
            Yattas Del Lana 1 1\n\
            ",
        ];
        let variant = Variant2::default();
        assert_eq!(get_winner(&input, &variant).unwrap(), ("Eve Stojbs", 59))
    }
}
