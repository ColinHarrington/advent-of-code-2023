use crate::Device::{BackwardMirror, ForwardMirror, HorizontalSplitter, VerticalSplitter};
use crate::Heading::{Down, Left, Right, Up};
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, VecDeque};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Grid::from_input(input).illuminate((0usize, 0usize, Right)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from_input(input);
    let last = grid.size - 1;
    (0..=last)
        .flat_map(|i| vec![(last, i, Up), (0, i, Down), (i, last, Left), (i, 0, Right)])
        .map(|start| grid.illuminate(start))
        .max()
}

type Position = (usize, usize);
type Light = (usize, usize, Heading);

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}
enum Device {
    HorizontalSplitter,
    VerticalSplitter,
    ForwardMirror,
    BackwardMirror,
}

impl Device {
    fn operate(&self, heading: Heading) -> Vec<Heading> {
        match (self, heading) {
            (HorizontalSplitter, Up) | (HorizontalSplitter, Down) => vec![Left, Right],
            (VerticalSplitter, Left) | (VerticalSplitter, Right) => vec![Up, Down],
            (ForwardMirror, Up) => vec![Right],
            (ForwardMirror, Down) => vec![Left],
            (ForwardMirror, Left) => vec![Down],
            (ForwardMirror, Right) => vec![Up],

            (BackwardMirror, Up) => vec![Left],
            (BackwardMirror, Down) => vec![Right],
            (BackwardMirror, Left) => vec![Up],
            (BackwardMirror, Right) => vec![Down],
            (_, h) => vec![h],
        }
    }
}
struct Grid {
    devices: HashMap<Position, Device>,
    size: usize,
}
impl Grid {
    fn from_input(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let size = lines.len();
        let width = lines.first().unwrap_or(&"").len();
        assert_eq!(size, width,);
        let devices: HashMap<Position, Device> =
            HashMap::from_iter(lines.into_iter().enumerate().flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, ch)| ((row, col), ch))
                    .filter_map(|(position, ch)| match ch {
                        '-' => Some((position, HorizontalSplitter)),
                        '|' => Some((position, VerticalSplitter)),
                        '/' => Some((position, ForwardMirror)),
                        '\\' => Some((position, BackwardMirror)),
                        _ => None,
                    })
            }));
        Grid { devices, size }
    }

    fn illuminate(&self, start: Light) -> usize {
        let mut word: BTreeSet<Light> = BTreeSet::new();
        let mut beams: VecDeque<Light> = VecDeque::from(vec![start]);

        while let Some((row, col, heading)) = beams.pop_front() {
            if word.insert((row, col, heading)) {
                for beam in self
                    .devices
                    .get(&(row, col))
                    .map(|device| device.operate(heading))
                    .unwrap_or(vec![heading])
                    .into_iter()
                    .map(|heading| match heading {
                        Up => (row.checked_sub(1), Some(col), heading),
                        Down => (row.checked_add(1), Some(col), heading),
                        Left => (Some(row), col.checked_sub(1), heading),
                        Right => (Some(row), col.checked_add(1), heading),
                    })
                    .filter_map(|entry| match entry {
                        (Some(row), Some(col), heading) if row < self.size && col < self.size => {
                            Some((row, col, heading))
                        }
                        _ => None,
                    })
                {
                    beams.push_back(beam);
                }
            }
        }
        word.into_iter()
            .map(|(row, col, _)| (row, col))
            .unique()
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
