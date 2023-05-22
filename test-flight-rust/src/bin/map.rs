use std::io::{Read, stdin};
use std::str::FromStr;
use anyhow::Result;
use test_flight_rust::map::{Planet, plan_route};
use test_flight_rust::tpa::marvin_tpa;

fn main() -> Result<()> {
    println!("Input map:");
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let planets: Vec<Planet> = input.lines()
        .filter_map(|l| Some(l.trim()).filter(|l| !l.is_empty()))
        .map(Planet::from_str)
        .collect::<Result<Vec<_>>>()?;
    let route = plan_route(&planets);
    println!("{}", route);
    println!("{}", marvin_tpa(route)?);
    Ok(())

}
