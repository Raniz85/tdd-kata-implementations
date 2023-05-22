use std::borrow::ToOwned;
use std::ops::Deref;
use std::str::FromStr;
use anyhow::{Result, Error, anyhow};
use once_cell::sync::Lazy;
use ordered_float::NotNan;
use num_traits::Zero;

#[derive(Clone, Debug, PartialEq)]
pub struct Point(pub [NotNan<f32>; 4]);

impl From<[NotNan<f32>; 4]> for Point {

    fn from(source: [NotNan<f32>; 4]) -> Point {
        Point(source)
    }
}

impl From<[f32; 4]> for Point {

    fn from(source: [f32; 4]) -> Point {
        Point(source.map(|it| NotNan::new(it).unwrap_or(NotNan::zero())))
    }
}

impl Point {

    pub fn distance(&self, b: &Point) -> NotNan<f32> {
        NotNan::new(b.0.iter().zip(self.0.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt())
            .unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Planet {
    pub name: String,
    pub location: Point,
}

impl FromStr for Planet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: [&str; 2] = s.splitn(2, '(')
            .map(str::trim)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow!("Malformed planet string '{}'", s))?;
        let name = parts[0];
        let point: [f32; 4] = parts[1].trim_matches(')')
            .split(",")
            .map(|c| Ok(f32::from_str(c.trim())?))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| anyhow!("Malformed planet string '{}'", s))?;
        Ok(Planet {
            name: name.to_owned(),
            location: point.into(),
        })
    }
}

impl Planet {
    pub fn distance(&self, other: &Planet) -> NotNan<f32> {
        self.location.distance(&other.location)
    }
}

static SOL: Lazy<Planet> = Lazy::new(|| Planet {
    name: "SOL".to_owned(),
    location: [0f32, 0f32, 0f32, 0f32].into(),
});

pub fn plan_route(planets: &[Planet]) -> String {
    let mut prev = SOL.deref();
    let mut route = "SOL\n".to_string();
    let mut planets: Vec<&Planet> = planets.iter().collect();
    while !planets.is_empty() {
        let (index, closest) = planets.iter()
            .enumerate()
            .min_by_key(|(index, planet)| planet.distance(prev))
            .unwrap(); // This will always be something since we check is_empty above
        route.push_str(&closest.name);
        route.push('\n');
        prev = *closest;
        planets.remove(index);
    }
    route.push_str("SOL");
    route
}


#[cfg(test)]
mod tests {
    use std::fmt::Alignment::Left;
    use std::str::FromStr;
    use crate::map::{plan_route, Planet};
    use super::Point;

    #[test]
    fn test_into_planet() {
        // Given an input planet as a string
        let input = "BETA VOLANTIS (3.4019889954534435, -44.01794341149888, -98.52628216246059, 0.162)";

        // When the string is parsed as a planet
        let planet: Planet = Planet::from_str(input).unwrap();

        // Then the planet is as expected
        assert_eq!(Planet {
            name: "BETA VOLANTIS".to_owned(),
            location: [
                3.4019889954534435,
                -44.01794341149888,
                -98.52628216246059,
                0.162
            ].into(),
        }, planet);
    }

    #[test]
    fn test_calculate_distance() {
        // Given two points with four coordinates
        let pointA: Point = [1f32, 2f32, 3f32, 4f32].into();
        let pointB: Point = [2f32, 4f32, 6f32, 8f32].into();

        // When the distance between them is calculated
        let d = pointA.distance(&pointB);

        // Then it is as expected
        assert_eq!(d, 5.477225575051661)
    }

    #[test]
    fn test_plan_route_one_planet() {
        // Given one planet
        let planet = Planet {
            name: "BETA VOLANTIS".to_owned(),
            location: [
                3.4019889954534435,
                -44.01794341149888,
                -98.52628216246059,
                0.162
            ].into(),
        };

        // When a route is planned with that planet
        let route = plan_route(&[planet]);

        // Then the route ss SOL, BETA VOLANTIS, SOL
        assert_eq!("SOL\n\
                   BETA VOLANTIS\n\
                   SOL", route)
    }

    #[test]
    fn test_plan_route_two_planets() {
        // Given one planet
        let planet_a = Planet {
            name: "ALPHA".to_owned(),
            location: [
                1f32,
                1f32,
                1f32,
                1f32,
            ].into(),
        };
        let planet_b = Planet {
            name: "BETA".to_owned(),
            location: [
                2f32,
                2f32,
                2f32,
                2f32,
            ].into(),
        };
        // When a route is planned with that planet
        let route = plan_route(&[planet_a, planet_b]);

        // Then the route ss SOL, BETA VOLANTIS, SOL
        assert_eq!("SOL\n\
                   ALPHA\n\
                   BETA\n\
                   SOL", route)
    }

    #[test]
    fn test_plan_route_three_planets() {
        // Given one planet
        let planet_a = Planet {
            name: "ALPHA".to_owned(),
            location: [
                1f32,
                1f32,
                1f32,
                1f32,
            ].into(),
        };
        let planet_b = Planet {
            name: "BETA".to_owned(),
            location: [
                -1f32,
                -1f32,
                -1f32,
                -1f32,
            ].into(),
        };
        let planet_c = Planet {
            name: "GAMMA".to_owned(),
            location: [
                1f32,
                1f32,
                1f32,
                2f32,
            ].into(),
        };
        // When a route is planned with that planet
        let route = plan_route(&[planet_a, planet_b, planet_c]);

        // Then the route ss SOL, BETA VOLANTIS, SOL
        assert_eq!("SOL\n\
                   ALPHA\n\
                   GAMMA\n\
                   BETA\n\
                   SOL", route)
    }

}
