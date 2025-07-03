use crate::Direction::{East, North, South, West};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let platform = Platform {
        data: input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
        direction: West,
    };
    Some(platform.tilt_north().score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let platform = Platform {
        data: input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
        direction: West,
    };
    let p = platform.tilt_north().tilt_north().tilt_north().tilt_north();
    println!("{p}");
    // let p = cycle(platform).score();
    None
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Platform {
    data: Vec<Vec<char>>,
    direction: Direction,
}
impl Platform {
    fn from_input(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let size = data.len();
        Platform {
            data: (0..size)
                .map(|row| {
                    (0..size)
                        .map(|col| data[col][size - 1 - row])
                        .collect_vec()
                })
                .collect(),
            direction:North
        }
    } 
    fn score(&self) -> usize {
        let size = self.data.len();
        self.data
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(col, ch)| match ch {
                        'O' => match self.direction {
                            // Some(size - col),
                            North => Some(size - col),
                            East => Some(size - row),
                            _ => unimplemented!("Never gonna give you up"),
                        },
                        _ => None,
                    })
            })
            .sum::<usize>()
    }
    
    /// Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east.
    fn cycle(&self) -> Platform {
        self.clone()
    }
    fn rotate_right(&mut self) {
        let size = self.data.len();
        self.data =  (0..size)
                .map(|row| {
                    (0..size)
                        .map(|col| self.data[col][size - 1 - row])
                        .collect_vec()
                })
                .collect();
        self.direction = self.direction.rotate_right();
    }

    fn tilt_north(&self) -> Platform {
        let size = self.data.len();
        let p = Platform {
            data: (0..size)
                .map(|row| {
                    roll(
                        (0..size)
                            .map(|col| self.data[col][size - 1 - row])
                            .collect_vec(),
                    )
                })
                .collect(),
            direction: self.direction.rotate_right(),
        };
        println!("{p}");
        p
    }
}
impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .data
            .iter()
            .map(|row| row.iter().join(" "))
            .collect_vec();
        writeln!(f, "{:?}\n{}\n", self.direction, lines.join("\n"))
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn rotate(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

fn rotate_right(platform: Platform) -> Platform {
    let size = platform.data.len();
    let data: Vec<Vec<char>> = (0..size)
        .map(|row| {
            roll(
                (0..size)
                    .map(|col| platform.data[col][size - 1 - row])
                    .collect_vec(),
            )
        })
        .collect();

    Platform {
        data,
        direction: platform.direction.rotate_right(),
    }
}
fn roll(line: Vec<char>) -> Vec<char> {
    line.split_inclusive(|ch| *ch == '#')
        .map(|group| {
            group
                .into_iter()
                .sorted_by(stone_sort)
                .map(|x| *x)
                .collect_vec()
        })
        .concat()
}

fn stone_sort(a: &&char, b: &&char) -> Ordering {
    match a {
        ch if ch == b => Ordering::Equal,
        'O' => Ordering::Less,
        '#' => Ordering::Greater,
        '.' => match b {
            '#' => Ordering::Less,
            _ => Ordering::Equal,
        },
        _ => Ordering::Equal,
    }
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
}
