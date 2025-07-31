use crate::parse::parse;
use itertools::Itertools;
use std::ops::RangeInclusive;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}
impl From<(i64, i64, i64, i64, i64, i64)> for Hailstone {
    fn from((x, y, z, dx, dy, dz): (i64, i64, i64, i64, i64, i64)) -> Self {
        Hailstone {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(intersections_2d(
        &parse(input),
        200000000000000f64..=400000000000000f64,
    ))
}

pub fn intersections_2d(stones: &[Hailstone], test_range: RangeInclusive<f64>) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| intersection2d(a, b))
        .filter(|(x, y)| test_range.contains(x) && test_range.contains(y))
        .count()
}

fn intersection2d(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let (x1, y1, dx1, dy1) = (a.x as f64, a.y as f64, a.dx as f64, a.dy as f64);
    let (x2, y2, dx2, dy2) = (b.x as f64, b.y as f64, b.dx as f64, b.dy as f64);
    if dx1 * dy2 - dy1 * dx2 == 0f64 {
        None
    } else {
        let m1 = dy1 / dx1;
        let q1 = -m1 * x1 + y1;
        let m2 = dy2 / dx2;
        let q2 = -m2 * x2 + y2;

        let xc = (q2 - q1) / (m1 - m2);
        let yc = m1 * xc + q1;

        let tc1 = (xc - x1) / dx1;
        let tc2 = (xc - x2) / dx2;
        if tc1 < 0f64 || tc2 < 0f64 {
            None
        } else {
            Some((xc, yc))
        }
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

mod parse {
    use crate::Hailstone;
    use nom::branch::alt;
    use nom::character::complete;
    use nom::character::complete::{char, line_ending, space0};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::IResult;

    pub fn parse(input: &str) -> Vec<Hailstone> {
        hailstones(input).unwrap().1
    }

    fn hailstones(input: &str) -> IResult<&str, Vec<Hailstone>> {
        separated_list1(line_ending, hailstone)(input)
    }

    fn hailstone(input: &str) -> IResult<&str, Hailstone> {
        map(
            separated_list1(
                delimited(space0, alt((char(','), char('@'))), space0),
                complete::i64,
            ),
            |list| Hailstone::from((list[0], list[1], list[2], list[3], list[4], list[5])),
        )(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let stones = parse(&advent_of_code::template::read_file("examples", DAY));
        let result = intersections_2d(&stones, 7f64..=27f64);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
