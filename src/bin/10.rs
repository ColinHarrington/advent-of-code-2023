use crate::Direction::{East, North, South, West};
use crate::PipeType::{
    BendNorthEast, BendNorthWest, BendSouthEast, BendSouthWest, Horizontal, Vertical,
};
use crate::Tile::{Ground, Pipe, Start};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    Some(PipeMaze::from_input(input).loop_length() / 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    let maze = PipeMaze::from_input(input);
    let loop_map = maze.loop_map();

    Some(
        maze.tiles
            .iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .fold(
                        (0usize, false, None),
                        |(area, inside, prior), (col, _)| match loop_map.get(&Point { row, col }) {
                            None => (if inside { area + 1 } else { area }, inside, prior),
                            Some(tile) => match *tile {
                                Pipe(Horizontal) => (area, inside, prior),
                                Pipe(Vertical) => (area, !inside, None),
                                Pipe(pipe_type) => match (prior, pipe_type) {
                                    (Some(BendNorthEast), BendSouthWest) => (area, !inside, None),
                                    (Some(BendSouthEast), BendNorthWest) => (area, !inside, None),
                                    _ => (area, inside, Some(pipe_type)),
                                },
                                _ => panic!("Invalid state"),
                            },
                        },
                    )
                    .0
            })
            .sum::<usize>() as u64,
    )
}

fn start_mapping(start: &Point, other: &Point) -> Direction {
    if start.row < other.row {
        South
    } else if start.row > other.row {
        North
    } else if start.col > other.col {
        West
    } else {
        East
    }
}
fn determine_starting_pipe(pipe_loop: &[Point]) -> Tile {
    let start = pipe_loop.first().unwrap();
    match (
        start_mapping(start, pipe_loop.get(1).unwrap()),
        start_mapping(start, pipe_loop.last().unwrap()),
    ) {
        (North, South) | (South, North) => Pipe(Vertical),
        (East, West) | (West, East) => Pipe(Horizontal),
        (North, East) | (East, North) => Pipe(BendNorthEast),
        (North, West) | (West, North) => Pipe(BendNorthWest),
        (South, East) | (East, South) => Pipe(BendSouthEast),
        (South, West) | (West, South) => Pipe(BendSouthWest),
        _ => panic!("Invalid combination"),
    }
}
#[derive(Clone)]
struct PipeCombo(PipeType, PipeType);
impl From<(PipeType, PipeType)> for PipeCombo {
    fn from((a, b): (PipeType, PipeType)) -> Self {
        PipeCombo(a, b)
    }
}

impl PartialEq<Self> for PipeCombo {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 || self.0 == other.1) && (self.1 == other.0 || self.1 == other.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
                Vertical => '│',
                Horizontal => '─',
                BendNorthEast => '└',
                BendNorthWest => '┘',
                BendSouthEast => '┌',
                BendSouthWest => '┐',
            }
        )
    }
}

impl PipeType {
    fn outgoing_direction(&self, incoming: Direction) -> Result<Direction, String> {
        match (self, incoming) {
            (Vertical, North) => Ok(North),
            (Vertical, South) => Ok(South),
            (Horizontal, East) => Ok(East),
            (Horizontal, West) => Ok(West),
            (BendNorthEast, South) => Ok(East),
            (BendNorthEast, West) => Ok(North),
            (BendNorthWest, South) => Ok(West),
            (BendNorthWest, East) => Ok(North),
            (BendSouthEast, North) => Ok(East),
            (BendSouthEast, West) => Ok(South),
            (BendSouthWest, East) => Ok(South),
            (BendSouthWest, North) => Ok(West),
            _ => Err("Invalid Direction".to_string()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Ground,
    Pipe(PipeType),
}
impl Tile {
    fn accepts(&self, direction: Direction) -> bool {
        match self {
            Start => true,
            Ground => false,
            Pipe(pipe_type) => match pipe_type {
                Vertical => matches!(direction, North | South),
                Horizontal => matches!(direction, East | West),
                BendNorthEast => matches!(direction, South | West),
                BendNorthWest => matches!(direction, South | East),
                BendSouthEast => matches!(direction, North | West),
                BendSouthWest => matches!(direction, North | East),
            },
        }
    }
}
impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            'S' => Start,
            '.' => Ground,
            '|' => Pipe(Vertical),
            '-' => Pipe(Horizontal),
            'L' => Pipe(BendNorthEast),
            'J' => Pipe(BendNorthWest),
            '7' => Pipe(BendSouthWest),
            'F' => Pipe(BendSouthEast),
            'O' | 'I' => Ground, //Testing examples!
            _ => panic!("Not a tile."),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Start => write!(f, "S"),
            Ground => write!(f, "."),
            Pipe(pipe_type) => std::fmt::Display::fmt(&pipe_type, f),
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
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

struct PipeMaze {
    tiles: Vec<Vec<Tile>>,
    data: HashMap<Point, Tile>,
    start: Point,
}
impl PipeMaze {
    fn from_input(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect_vec())
            .collect_vec();
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
                Start => Some(point),
                _ => None,
            })
            .unwrap();
        PipeMaze { tiles, data, start }
    }

    fn loop_length(&self) -> u64 {
        self.load_loop().len() as u64
    }

    fn loop_map(&self) -> HashMap<Point, Tile> {
        let pipe_loop = self.load_loop();
        let start = determine_starting_pipe(&pipe_loop);
        HashMap::from_iter(pipe_loop.into_iter().map(|p| {
            (
                p,
                match *self.data.get(&p).unwrap() {
                    Start => start,
                    tile => tile,
                },
            )
        }))
    }

    fn load_loop(&self) -> Vec<Point> {
        [North, South, East, West]
            .into_iter()
            .find_map(|direction| self.find_loop(direction))
            .unwrap()
    }

    fn find_loop(&self, direction: Direction) -> Option<Vec<Point>> {
        let mut path = vec![self.start];
        // let pipe_walker = self.iter(direction);
        for point in self.iter(direction) {
            if point == self.start {
                return Some(path);
            }
            path.push(point);
        }
        None
    }

    fn iter(&self, direction: Direction) -> PipeWalker {
        PipeWalker {
            current: self.start,
            direction,
            maze: self,
        }
    }
}
impl Display for PipeMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tiles
                .iter()
                .map(|row| row.iter().map(Tile::to_string).join(""))
                .join("\n")
        )
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
                        Ground => None,
                        Start => Some(self.current),
                        Pipe(pipe_type) => match tile.accepts(self.direction) {
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
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_example("10"));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two(&advent_of_code::template::read_example("10c"));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_d() {
        let result = part_two(&advent_of_code::template::read_example("10d"));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_e() {
        let result = part_two(&advent_of_code::template::read_example("10e"));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_f() {
        let result = part_two(&advent_of_code::template::read_example("10f"));
        assert_eq!(result, Some(10));
    }
}
