use crate::parse::read;
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = read(input);
    Some(
        races
            .times
            .into_iter()
            .zip(races.records)
            .map(|(time, record)| wins(time, record))
            .collect_vec()
            .iter()
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = read(input);
    let time: u64 =
        u64::from_str(races.times.iter().map(u64::to_string).join("").as_str()).unwrap();
    let record: u64 =
        u64::from_str(races.records.iter().map(u64::to_string).join("").as_str()).unwrap();
    Some(wins(time, record))
}

/**
Movement function
f(x) = x * (time - x)
f(x) = -x² + time*x

Standard form:
(ax² + bx + c = 0)
0 = -x² - time*x - record
x = (-b ± √(b² - 4ac)) / 2a

a = -1
b = time
c = -record
*/
fn wins(time: u64, record: u64) -> u64 {
    let (r1, r2) = solve_quadratic(-1, time as i64, -(record as i64));

    let lower = match r1.ceil() as u64 {
        root if root * (time - root) == record => root + 1,
        root => root,
    };
    let upper = match r2.floor() as u64 {
        root if root * (time - root) == record => root - 1,
        root => root,
    };

    upper - lower + 1
}

/**
  Solving via quadratic formula:
    x = (-b ± √(b² - 4ac)) / 2a
  No indeterminate roots here.
*/
fn solve_quadratic(a: i64, b: i64, c: i64) -> (f64, f64) {
    let b2minus4ac = b * b - 4 * a * c;
    let sqrt_b2minus4ac = (b2minus4ac as f64).sqrt();
    let two_a = 2.0 * a as f64;
    (
        (-b as f64 + sqrt_b2minus4ac) / two_a,
        (-b as f64 - sqrt_b2minus4ac) / two_a,
    )
}

struct Races {
    times: Vec<u64>,
    records: Vec<u64>,
}
mod parse {
    use crate::Races;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, multispace1, u64 as nom_u64};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, terminated};
    use nom::IResult;

    pub fn read(input: &str) -> Races {
        let (tail, races) = races(input.trim()).unwrap();
        assert_eq!("", tail);
        races
    }

    fn races(input: &str) -> IResult<&str, Races> {
        map(
            separated_pair(times, line_ending, distances),
            |(times, distances)| Races {
                times,
                records: distances,
            },
        )(input)
    }
    fn times(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(
            terminated(tag("Time:"), multispace1),
            separated_list1(multispace1, nom_u64),
        )(input)
    }
    fn distances(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(
            terminated(tag("Distance:"), multispace1),
            separated_list1(multispace1, nom_u64),
        )(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
