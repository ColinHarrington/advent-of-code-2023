use crate::Direction::{East, North, South, West};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut platform: Platform = input.into();
    platform.tilt(North);
    Some(score(platform.data))
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut platform: Platform = input.into();
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut rounds: Vec<Vec<Vec<char>>> = vec![];
    for c in 0..1000000000usize {
        platform.cycle();
        let data = platform.data.clone();
        rounds.push(data.clone());
        if let Some(existing) = cache.insert(data, c) {
            let cycle_length = c - existing;
            let index = (1000000000usize - existing) % cycle_length + (existing - 1);
            return Some(score(rounds[index].clone()));
        }
    }

    None
}

#[derive(Hash, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
struct Platform {
    data: Vec<Vec<char>>,
    mappings: HashMap<Direction, Vec<Vec<(usize, usize)>>>,
}
impl From<&str> for Platform {
    fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let size = data.len();
        let mappings: HashMap<Direction, Vec<Vec<(usize, usize)>>> = HashMap::from([
            (
                North,
                (0..size)
                    .map(|col| (0..size).map(|row| (row, col)).collect_vec())
                    .collect_vec(),
            ),
            (
                East,
                (0..size)
                    .map(|row| (0..size).rev().map(|col| (row, col)).collect_vec())
                    .collect_vec(),
            ),
            (
                South,
                (0..size)
                    .map(|col| (0..size).rev().map(|row| (row, col)).collect_vec())
                    .collect_vec(),
            ),
            (
                West,
                (0..size)
                    .map(|row| (0..size).map(|col| (row, col)).collect_vec())
                    .collect_vec(),
            ),
        ]);

        Platform { data, mappings }
    }
}
impl Platform {
    fn cycle(&mut self) {
        self.tilt(North);
        self.tilt(West);
        self.tilt(South);
        self.tilt(East);
    }
    fn tilt(&mut self, direction: Direction) {
        let mapping = self.mappings.get(&direction).unwrap();
        for group in mapping {
            let mut spaces: VecDeque<(usize, usize)> = VecDeque::new();
            for (row, col) in group {
                let ch = self.data[*row][*col];
                match ch {
                    'O' => match spaces.pop_front() {
                        None => {}
                        Some((r, c)) => {
                            self.data[r][c] = 'O';
                            self.data[*row][*col] = '.';
                            spaces.push_back((*row, *col));
                        }
                    },
                    '#' => spaces.clear(),
                    '.' => spaces.push_back((*row, *col)),
                    _ => panic!("Unexpected character"),
                }
            }
        }
    }
}

fn score(data: Vec<Vec<char>>) -> usize {
    let size = data.len();
    data.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().filter_map(move |ch| match ch {
                'O' => Some(size - row),
                _ => None,
            })
        })
        .sum::<usize>()
}
#[allow(dead_code)]
fn print_platform(data: Vec<Vec<char>>) {
    let lines: Vec<String> = data.iter().map(|row| row.iter().join("")).collect_vec();
    println!("{}\n", lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
    #[test]
    fn test_cycle() {
        let mut platform =
            Platform::from(advent_of_code::template::read_file("examples", DAY).as_str());
        let cycles = [
            &advent_of_code::template::read_example(&format!("{}-cycle-1", DAY)),
            &advent_of_code::template::read_example(&format!("{}-cycle-2", DAY)),
            &advent_of_code::template::read_example(&format!("{}-cycle-3", DAY)),
        ];

        platform.cycle();
        // data = cycle(data.clone());
        assert_eq!(platform.data, to_data(cycles[0]));

        platform.cycle();
        assert_eq!(platform.data, to_data(cycles[1]));

        platform.cycle();
        assert_eq!(platform.data, to_data(cycles[2]));
    }

    fn to_data(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec()
    }
}
