use crate::Direction::{East, North, South, West};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let maze = PipeMaze::from_input(input);
    [North, South, East, West]
        .into_iter()
        .find_map(|direction| maze.find_loop(direction))
        .map(|points| ((points.len() + 1) / 2) as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum PipeType {
    Vertical,      // │
    Horizontal,    // ─
    BendNorthEast, // └
    BendNorthWest, // ┘
    BendSouthEast, // ┌
    BendSouthWest, // ┐
}

impl Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PipeType::Vertical => '│',
                PipeType::Horizontal => '─',
                PipeType::BendNorthEast => '└',
                PipeType::BendNorthWest => '┘',
                PipeType::BendSouthEast => '┌',
                PipeType::BendSouthWest => '┐',
            }
        )
    }
}

// (incoming, outgoing)
impl PipeType {
    fn outgoing_direction(&self, incoming: Direction) -> Result<Direction, String> {
        match (self, incoming) {
            (PipeType::Vertical, North) => Ok(North),
            (PipeType::Vertical, South) => Ok(South),
            (PipeType::Horizontal, East) => Ok(East),
            (PipeType::Horizontal, West) => Ok(West),
            (PipeType::BendNorthEast, South) => Ok(East),
            (PipeType::BendNorthEast, West) => Ok(North),
            (PipeType::BendNorthWest, South) => Ok(West),
            (PipeType::BendNorthWest, East) => Ok(North),
            (PipeType::BendSouthEast, North) => Ok(East),
            (PipeType::BendSouthEast, West) => Ok(South),
            (PipeType::BendSouthWest, East) => Ok(South),
            (PipeType::BendSouthWest, North) => Ok(West),
            _ => Err("Invalid Direction".to_string()),
        }
    }
}
#[derive(Debug, Clone)]
enum Tile {
    Start,
    Ground,
    Pipe(PipeType),
}
impl Tile {
    fn accepts(&self, direction: Direction) -> bool {
        match self {
            Tile::Start => true,
            Tile::Ground => false,
            Tile::Pipe(pipe_type) => match pipe_type {
                PipeType::Vertical => matches!(direction, North | South),
                PipeType::Horizontal => matches!(direction, East | West),
                PipeType::BendNorthEast => matches!(direction, South | West),
                PipeType::BendNorthWest => matches!(direction, South | East),
                PipeType::BendSouthEast => matches!(direction, North | West),
                PipeType::BendSouthWest => matches!(direction, North | East),
            },
        }
    }
}
impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            'S' => Tile::Start,
            '.' => Tile::Ground,
            '|' => Tile::Pipe(PipeType::Vertical),
            '-' => Tile::Pipe(PipeType::Horizontal),
            'L' => Tile::Pipe(PipeType::BendNorthEast),
            'J' => Tile::Pipe(PipeType::BendNorthWest),
            '7' => Tile::Pipe(PipeType::BendSouthWest),
            'F' => Tile::Pipe(PipeType::BendSouthEast),
            _ => panic!("Not a tile."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn step(&self, direction: Direction) -> Result<Point, String> {
        let (row, col) = match direction {
            North => (self.row.checked_sub(1), Some(self.col)),
            East => (Some(self.row), self.col.checked_add(1)),
            South => (self.row.checked_add(1), Some(self.col)),
            West => (Some(self.row), self.col.checked_sub(1)),
        };
        match (row, col) {
            (Some(row), Some(col)) => Ok(Point { row, col }),
            _ => Err("Cannot index off grid".to_string()),
        }
    }
}

struct PipeMaze {
    data: HashMap<Point, Tile>,
    start: Point,
}
impl PipeMaze {
    fn from_input(input: &str) -> Self {
        let data: HashMap<Point, Tile> =
            HashMap::from_iter(input.trim().lines().enumerate().flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| (Point { row, col }, Tile::from(c)))
                    .collect_vec()
            }));
        let start = data
            .iter()
            .find_map(|(&point, tile)| match tile {
                Tile::Start => Some(point),
                _ => None,
            })
            .unwrap();
        PipeMaze { data, start }
    }

    fn find_loop(&self, direction: Direction) -> Option<Vec<Point>> {
        let mut path = vec![];
        let pipe_walker = self.walker(direction);
        for point in pipe_walker {
            if point == self.start {
                return Some(path);
            }
            path.push(point);
        }
        None
    }

    fn walker(&self, direction: Direction) -> PipeWalker {
        PipeWalker {
            current: self.start,
            direction,
            maze: self,
        }
    }
}

struct PipeWalker<'a> {
    current: Point,
    direction: Direction,
    maze: &'a PipeMaze,
}

impl Iterator for PipeWalker<'_> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.step(self.direction) {
            Err(_) => None,
            Ok(point) => {
                self.current = point;
                match self.maze.data.get(&self.current) {
                    Some(tile) => match tile {
                        Tile::Ground => None,
                        Tile::Start => Some(self.current),
                        Tile::Pipe(pipe_type) => match tile.accepts(self.direction) {
                            true => match pipe_type.outgoing_direction(self.direction) {
                                Ok(outgoing_direction) => {
                                    self.direction = outgoing_direction;
                                    Some(self.current)
                                }
                                Err(_) => None,
                            },
                            false => None,
                        },
                    },
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_example("10b"));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
