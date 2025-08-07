use crate::parse::parse;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{RangeInclusive, Sub};

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Hailstone {
    x: i128,
    y: i128,
    z: i128,
    dx: i128,
    dy: i128,
    dz: i128,
}
impl From<(i128, i128, i128, i128, i128, i128)> for Hailstone {
    fn from((x, y, z, dx, dy, dz): (i128, i128, i128, i128, i128, i128)) -> Self {
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
impl From<Hailstone> for (i128, i128, i128, i128, i128, i128) {
    fn from(val: Hailstone) -> Self {
        (val.x, val.y, val.z, val.dx, val.dy, val.dz)
    }
}
impl Sub for Hailstone {
    type Output = Hailstone;

    fn sub(self, rhs: Self) -> Self::Output {
        Hailstone {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
            dz: self.dz - rhs.dz,
        }
    }
}
impl Display for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Hailstone {
    fn velocity_delta(&self, other: &Hailstone, time: i128) -> Hailstone {
        Hailstone {
            x: 0,
            y: 0,
            z: 0,
            dx: (other.x - self.x) / time,
            dy: (other.y - self.y) / time,
            dz: (other.z - self.z) / time,
        }
    }
    fn position_at(&self, time: i128) -> Self {
        Hailstone {
            x: self.dx * time + self.x,
            y: self.dy * time + self.y,
            z: self.dz * time + self.z,
            ..*self
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

pub fn part_two(input: &str) -> Option<i128> {
    let stones = parse(input);
    let [a, b, c] = stones[..3] else {
        panic!("Method requires three stones")
    };
    let t1 = intersection_time(&[a, b, c]);
    let t2 = intersection_time(&[b, a, c]);
    Some(
        [(t1, a), (t2, b)]
            .into_iter()
            .map(|(time, stone)| stone.position_at(time))
            .tuples()
            .map(|(collision_1, collision_2)| {
                collision_1
                    .velocity_delta(&collision_2, t2 - t1)
                    .position_at(t1)
            })
            .map(|delta| a.position_at(t1).sub(delta))
            .map(|s| s.x + s.y + s.z)
            .next()
            .unwrap(),
    )
}
fn intersection_time(stones: &[Hailstone; 3]) -> i128 {
    let (x1, y1, z1, dx1, dy1, dz1) = stones[0].into();
    let (x2, y2, z2, dx2, dy2, dz2) = stones[1].into();
    let (x3, y3, z3, dx3, dy3, dz3) = stones[2].into();
    let yz = y1 * (z2 - z3) + y2 * (-z1 + z3) + y3 * (z1 - z2);
    let xz = x1 * (-z2 + z3) + x2 * (z1 - z3) + x3 * (-z1 + z2);
    let xy = x1 * (y2 - y3) + x2 * (-y1 + y3) + x3 * (y1 - y2);
    let dxdy = dx1 * (dy2 - dy3) + dx2 * (-dy1 + dy3) + dx3 * (dy1 - dy2);
    let dxdz = dx1 * (-dz2 + dz3) + dx2 * (dz1 - dz3) + dx3 * (-dz1 + dz2);
    let dydz = dy1 * (dz2 - dz3) + dy2 * (-dz1 + dz3) + dy3 * (dz1 - dz2);
    ((dx2 - dx3) * yz + (dy2 - dy3) * xz + (dz2 - dz3) * xy)
        / ((z2 - z3) * dxdy + (y2 - y3) * dxdz + (x2 - x3) * dydz)
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
                complete::i128,
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
        assert_eq!(result, Some(47));
    }
}
