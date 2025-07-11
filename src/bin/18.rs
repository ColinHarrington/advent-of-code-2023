use crate::parse::{part1_instruction, part2_instruction};
use crate::Direction::{Down, Left, Right, Up};
use itertools::Itertools;
use std::ops::{Add, Div};

advent_of_code::solution!(18);

type Instruction = (Direction, isize);
pub fn part_one(input: &str) -> Option<isize> {
    Some(calculate_area(part1_instructions(input)))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(calculate_area(part2_instructions(input)))
}

/// Shoelace + extra area
///
/// The area is calculated from the polygon points
/// https://en.wikipedia.org/wiki/Shoelace_formula#Exterior_Algebra
///
/// This area is measured from the center of the trenches, thus missing half the perimeter blocks
///  Plus one additional ¼ for the four outer corners unaccounted for in the original area.
///
///     extra_area = (perimeter / 2) + 1)
fn calculate_area(instructions: Vec<Instruction>) -> isize {
    let points: Vec<(isize, isize)> =
        instructions
            .iter()
            .fold(vec![], |mut points, (dir, steps)| {
                let (x, y) = points.last().unwrap_or(&(0isize, 0isize));
                points.push(match dir {
                    Left => (x - steps, *y),
                    Right => (x + steps, *y),
                    Up => (*x, y - steps),
                    Down => (*x, y + steps),
                });
                points
            });
    let perimeter = instructions.iter().map(|(_, steps)| *steps).sum::<isize>();
    shoelace(&points) + perimeter.div(2).add(1)
}

/// Shoelace Area
///
/// Requires the points to be in consecutive order
/// https://en.wikipedia.org/wiki/Shoelace_formula#Exterior_Algebra
///
///     2A = ∑ ( x * yᵢ₊₁ - y * xᵢ₊₁ )
///
/// or ...
///
///     A = ( ∑ ( x * yᵢ₊₁ - y * xᵢ₊₁ ) ) / 2
fn shoelace(points: &[(isize, isize)]) -> isize {
    points
        .iter()
        .circular_tuple_windows()
        .fold(0isize, |area, ((x1, y1), (x2, y2))| {
            area + ((x1 * y2) - (x2 * y1))
        })
        .abs()
        .div(2)
}
fn part1_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| part1_instruction(line).unwrap().1)
        .collect()
}

fn part2_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| part2_instruction(line).unwrap().1)
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
mod parse {
    use crate::Direction::{Down, Left, Right, Up};
    use crate::{Direction, Instruction};
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take;
    use nom::character::complete;
    use nom::character::complete::{one_of, space1};
    use nom::combinator::map;
    use nom::sequence::{delimited, separated_pair, tuple};
    use nom::IResult;

    pub fn part1_instruction(input: &str) -> IResult<&str, Instruction> {
        map(instruction, |(part1, _)| part1)(input)
    }
    pub fn part2_instruction(input: &str) -> IResult<&str, Instruction> {
        map(instruction, |(_, part2)| part2)(input)
    }

    fn instruction(input: &str) -> IResult<&str, (Instruction, Instruction)> {
        separated_pair(
            separated_pair(direction, space1, steps),
            space1,
            hex_instruction,
        )(input)
    }

    fn direction(input: &str) -> IResult<&str, Direction> {
        map(one_of("RLUD"), |c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Not a valid direction"),
        })(input)
    }

    fn steps(input: &str) -> IResult<&str, isize> {
        map(complete::i64, |n| n as isize)(input)
    }

    fn hex_instruction(input: &str) -> IResult<&str, Instruction> {
        delimited(
            tag("(#"),
            map(tuple((take(5usize), one_of("0123"))), |(steps, dir)| {
                (
                    match dir {
                        '0' => Right,
                        '1' => Down,
                        '2' => Left,
                        '3' => Up,
                        _ => panic!("Invalid direction"),
                    },
                    isize::from_str_radix(steps, 16).unwrap(),
                )
            }),
            tag(")"),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
