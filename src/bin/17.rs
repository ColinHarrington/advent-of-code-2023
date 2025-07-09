use crate::Heading::{Down, Left, Right, Up};
use itertools::Itertools;
use pathfinding::directed::astar::astar;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let crucible = Crucible::from_input(input);

    let end = crucible.end_position();

    vec![Right, Down]
        .into_iter()
        .map(|heading| Node {
            position: Position { row: 0, col: 0 },
            heading,
            count: 1,
        })
        .filter_map(|start| {
            astar(
                &start,
                |node| crucible.successors(node, 0, 3),
                |node| (end.row - node.position.row + end.col - node.position.col) * 2,
                |node| node.position == end,
            )
        })
        // .inspect(|(path, _)| print_path(input, path))
        .map(|(_path, heat)| heat)
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let crucible = Crucible::from_input(input);

    let end = crucible.end_position();

    vec![Right, Down]
        .into_iter()
        .map(|heading| Node {
            position: Position { row: 0, col: 0 },
            heading,
            count: 1,
        })
        .filter_map(|start| {
            astar(
                &start,
                |node| crucible.successors(node, 4, 10),
                |node| (end.row - node.position.row + end.col - node.position.col) * 2,
                |node| node.position == end,
            )
        })
        // .inspect(|(path, _)| print_path(input, path))
        .map(|(_path, heat)| heat)
        .min()
}

struct Crucible {
    data: Vec<Vec<usize>>,
}
impl Crucible {
    fn from_input(input: &str) -> Self {
        Crucible {
            data: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
    fn end_position(&self) -> Position {
        Position::from((self.data.len() - 1, self.data.len() - 1))
    }
    fn heat(&self, position: Position) -> usize {
        self.data[position.row][position.col]
    }
    fn successors(&self, node: &Node, min: usize, max: usize) -> Vec<(Node, usize)> {
        let bounds = self.data.len() - 1;
        node.successors(bounds)
            .into_iter()
            .filter(|n| match n.heading == node.heading {
                true => node.count < max,
                false => node.count >= min,
            })
            .map(|node| (node, self.heat(node.position)))
            .collect()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Position {
    row: usize,
    col: usize,
}
impl Position {
    fn in_bounds(&self, max: usize) -> bool {
        self.row <= max && self.col <= max
    }
    fn right(&self, heading: Heading) -> Option<Position> {
        let (row, column) = match heading {
            Up => (Some(self.row), self.col.checked_add(1)),
            Down => (Some(self.row), self.col.checked_sub(1)),
            Left => (self.row.checked_sub(1), Some(self.col)),
            Right => (self.row.checked_add(1), Some(self.col)),
        };
        match (row, column) {
            (Some(row), Some(col)) => Some(Position { row, col }),
            _ => None,
        }
    }

    fn left(&self, heading: Heading) -> Option<Position> {
        let (row, column) = match heading {
            Up => (Some(self.row), self.col.checked_sub(1)),
            Down => (Some(self.row), self.col.checked_add(1)),
            Left => (self.row.checked_add(1), Some(self.col)),
            Right => (self.row.checked_sub(1), Some(self.col)),
        };
        match (row, column) {
            (Some(row), Some(col)) => Some(Position { row, col }),
            _ => None,
        }
    }

    fn straight(&self, heading: Heading) -> Option<Position> {
        let (row, column) = match heading {
            Up => (self.row.checked_sub(1), Some(self.col)),
            Down => (self.row.checked_add(1), Some(self.col)),
            Left => (Some(self.row), self.col.checked_sub(1)),
            Right => (Some(self.row), self.col.checked_add(1)),
        };
        match (row, column) {
            (Some(row), Some(col)) => Some(Position { row, col }),
            _ => None,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    position: Position,
    heading: Heading,
    count: usize,
}

impl Node {
    fn successors(&self, bounds: usize) -> Vec<Node> {
        vec![self.turn_right(), self.turn_left(), self.straight()]
            .into_iter()
            .filter_map(|node| match node {
                Some(node) if node.position.in_bounds(bounds) => Some(node),
                _ => None,
            })
            .collect_vec()
    }
    fn turn_right(&self) -> Option<Node> {
        self.position.right(self.heading).map(|position| Node {
            position,
            heading: self.heading.right(),
            count: 1,
        })
    }

    fn turn_left(&self) -> Option<Node> {
        self.position.left(self.heading).map(|position| Node {
            position,
            heading: self.heading.left(),
            count: 1,
        })
    }
    fn straight(&self) -> Option<Node> {
        self.position.straight(self.heading).map(|position| Node {
            position,
            heading: self.heading,
            count: self.count + 1,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn right(&self) -> Heading {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    fn left(&self) -> Heading {
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
}

#[allow(dead_code)]
fn print_path(input: &str, path: &Vec<Node>) {
    let mut grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    for node in path {
        grid[node.position.row][node.position.col] = match node.heading {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        }
    }
    for line in grid {
        println!("{}", line.into_iter().collect::<String>());
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
