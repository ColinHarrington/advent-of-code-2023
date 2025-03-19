use crate::parse::read;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(|s| hash(s) as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes = LensBoxes::new();
    for (label, operation) in read(input) {
        match operation {
            Operation::Add(focal_length) => boxes.add(Lens {
                label: label.to_string(),
                focal_length,
            }),
            Operation::Remove => boxes.remove(label),
        }
    }
    Some(boxes.focusing_power())
}

fn hash(input: &str) -> u8 {
    input
        .chars()
        .map(|c| c as u8)
        .fold(0u8, |acc, c| acc.overflowing_add(c).0.overflowing_mul(17).0)
}
type Instruction<'a> = (&'a str, Operation);

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
struct LensBoxes {
    boxes: HashMap<u8, VecDeque<Lens>>,
}
impl LensBoxes {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }
    fn add(&mut self, lens: Lens) {
        let lens_box = self.boxes.entry(hash(&lens.label)).or_default();
        if let Some(existing) = lens_box.iter_mut().find(|other| lens.label == other.label) {
            existing.focal_length = lens.focal_length
        } else {
            lens_box.push_back(lens)
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(lens_box) = self.boxes.get_mut(&hash(label)) {
            lens_box.retain(|lens| lens.label != label);
        }
    }

    fn focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .map(|(box_number, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(slot, lens)| {
                        (*box_number as usize + 1) * (slot + 1) * (lens.focal_length as usize)
                    })
                    .sum::<usize>() as u32
            })
            .sum::<u32>()
    }
}
enum Operation {
    Add(u8),
    Remove,
}

mod parse {
    use crate::Instruction;
    use crate::Operation;
    use nom::branch::alt;
    use nom::character::complete;
    use nom::character::complete::alpha1;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, tuple};
    use nom::IResult;

    pub fn read(input: &str) -> Vec<Instruction> {
        instructions(input.trim()).unwrap().1
    }
    fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(complete::char(','), instruction)(input)
    }
    fn instruction(input: &str) -> IResult<&str, Instruction> {
        tuple((alpha1, operation))(input)
    }
    fn operation(input: &str) -> IResult<&str, Operation> {
        alt((operation_add, operation_remove))(input)
    }
    fn operation_add(input: &str) -> IResult<&str, Operation> {
        map(preceded(complete::char('='), complete::u8), Operation::Add)(input)
    }
    fn operation_remove(input: &str) -> IResult<&str, Operation> {
        map(complete::char('-'), |_| Operation::Remove)(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", hash("pc"));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
