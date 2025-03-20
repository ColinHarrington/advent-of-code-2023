use crate::parse::read;
use itertools::Itertools;
use num::integer::lcm;
use std::collections::BTreeMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, nodes) = read(input);
    let node_map: BTreeMap<&str, (&str, &str)> = BTreeMap::from_iter(nodes);

    let mut current = "AAA";
    let mut itr = instructions.iter().cycle();
    let mut steps: u64 = 0;
    while current != "ZZZ" {
        let (left, right) = node_map[current];
        current = match itr.next().unwrap() {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        steps += 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, nodes) = read(input);
    let node_map: BTreeMap<&str, (&str, &str)> = BTreeMap::from_iter(nodes);
    let starts = node_map
        .keys()
        .filter_map(|&key| match key.ends_with('A') {
            true => Some(key),
            false => None,
        })
        .collect_vec();
    let mut heads = starts.clone();
    let mut counts: Vec<usize> = vec![];
    let mut itr = instructions.iter().cycle();
    let mut steps: usize = 0;

    while !heads.is_empty() {
        let instruction = itr.next().unwrap();
        steps += 1;
        heads = heads
            .into_iter()
            .map(|current| match instruction {
                Instruction::Left => node_map[current].0,
                Instruction::Right => node_map[current].1,
            })
            .filter(|next| match next.ends_with('Z') {
                true => {
                    counts.push(steps);
                    false
                }
                false => true,
            })
            .collect_vec();
    }
    Some(counts.into_iter().reduce(lcm).unwrap() as u64)
}
type Node<'a> = (&'a str, (&'a str, &'a str));
enum Instruction {
    Left,
    Right,
}
impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Bad Robot: invalid instruction"),
        }
    }
}
type Wasteland<'a> = (Vec<Instruction>, Vec<Node<'a>>);

mod parse {
    use crate::{Instruction, Node, Wasteland};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alphanumeric1, char as nom_char, line_ending};
    use nom::combinator::map;
    use nom::multi::{count, many1, separated_list1};
    use nom::sequence::{preceded, separated_pair, terminated, tuple};
    use nom::IResult;

    pub fn read(input: &str) -> Wasteland {
        let (tail, wasteland) = wasteland(input.trim()).unwrap();
        assert_eq!("", tail);
        wasteland
    }
    fn wasteland(input: &str) -> IResult<&str, Wasteland> {
        tuple((terminated(instructions, count(line_ending, 2)), nodes))(input)
    }

    fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(instruction)(input)
    }
    fn instruction(input: &str) -> IResult<&str, Instruction> {
        map(alt((nom_char('L'), nom_char('R'))), Instruction::from)(input)
    }

    fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
        separated_list1(line_ending, node)(input)
    }
    fn node(input: &str) -> IResult<&str, Node> {
        separated_pair(alphanumeric1, tag(" = "), left_right)(input)
    }
    fn left_right(input: &str) -> IResult<&str, (&str, &str)> {
        preceded(
            nom_char('('),
            terminated(
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                nom_char(')'),
            ),
        )(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example("08b"));
        assert_eq!(result, Some(6));
    }
}
