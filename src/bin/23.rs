use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(23);

type Position = (usize, usize);
#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}
impl Grid {
    fn from_input(input: &str) -> Self {
        let data = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
        assert_eq!(
            data.len(),
            data[0].len(),
            "We are expecting the Grid to be square but it was {}x{}",
            data.len(),
            data[0].len()
        );
        let start = data
            .iter()
            .enumerate()
            .find_map(|(row, line)| {
                line.iter().enumerate().find_map(|(col, ch)| match ch {
                    '.' => Some((row, col)),
                    _ => None,
                })
            })
            .unwrap();
        let end = data
            .iter()
            .enumerate()
            .rev()
            .find_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .rev()
                    .find_map(|(col, ch)| match ch {
                        '.' => Some((row, col)),
                        _ => None,
                    })
            })
            .unwrap();
        Grid { data, start, end }
    }

    fn intersections(&self) -> HashSet<Position> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut intersections: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::from([self.start]);
        while let Some(head) = queue.pop_front() {
            if visited.insert(head) {
                let neighbors = self.neighbors(head);
                if neighbors.len() > 2 {
                    intersections.insert(head);
                }
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        intersections
    }

    fn neighbors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        [
            (row.checked_add(1), Some(col)),
            (row.checked_sub(1), Some(col)),
            (Some(row), col.checked_add(1)),
            (Some(row), col.checked_sub(1)),
        ]
        .into_iter()
        .filter_map(|n| match n {
            (Some(row), Some(col)) => Some((row, col)),
            _ => None,
        })
        .filter(|(row, col)| *row < self.data.len() && *col < self.data.len())
        .filter(|(row, col)| !matches!(self.data[*row][*col], '#'))
        .collect_vec()
    }

    fn directed_neighbors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        match self.data[row][col] {
            '.' => vec![
                (row.checked_add(1), Some(col)),
                (row.checked_sub(1), Some(col)),
                (Some(row), col.checked_add(1)),
                (Some(row), col.checked_sub(1)),
            ],
            '>' => vec![(Some(row), col.checked_add(1))],
            '<' => vec![(Some(row), col.checked_sub(1))],
            '^' => vec![(row.checked_sub(1), Some(col))],
            'v' => vec![(row.checked_add(1), Some(col))],
            _ => panic!("Unexpected entry"),
        }
        .into_iter()
        .filter_map(|n| match n {
            (Some(row), Some(col)) => Some((row, col)),
            _ => None,
        })
        .filter(|(row, col)| *row < self.data.len() && *col < self.data.len())
        .filter(|(row, col)| !matches!(self.data[*row][*col], '#'))
        .collect_vec()
    }

    fn to_graph(
        &self,
        directed: bool,
    ) -> (
        HashSet<Position>,
        HashMap<Position, HashMap<Position, usize>>,
    ) {
        let nodes: HashSet<Position> = HashSet::from_iter(
            vec![self.start, self.end]
                .into_iter()
                .chain(self.intersections().iter().copied()),
        );

        let edges: HashMap<Position, HashMap<Position, usize>> =
            HashMap::from_iter(nodes.clone().into_iter().map(|i| {
                let mut my_edges: HashMap<Position, usize> = HashMap::new();
                let mut queue: VecDeque<Vec<Position>> = VecDeque::from_iter([vec![i]]);
                while let Some(list) = queue.pop_front() {
                    if let Some(p) = list.last() {
                        for n in if directed {
                            self.directed_neighbors(*p)
                        } else {
                            self.neighbors(*p)
                        }
                        .into_iter()
                        .filter(|n| !list.contains(n))
                        {
                            if nodes.contains(&n) {
                                my_edges.insert(n, list.len());
                            } else {
                                queue.push_back(
                                    list.clone().into_iter().chain(vec![n]).collect_vec(),
                                );
                            }
                        }
                    }
                }
                (i, my_edges)
            }));
        (nodes, edges)
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from_input(input);
    let (_, edges) = grid.to_graph(true);
    let mut paths: Vec<Vec<Position>> = vec![];
    let mut q = VecDeque::from([vec![grid.start]]);
    while let Some(list) = q.pop_front() {
        if let Some(last) = list.last() {
            if last == &grid.end {
                paths.push(list);
            } else {
                for p in edges.get(last).unwrap().keys() {
                    q.push_back(list.clone().into_iter().chain(vec![*p]).collect_vec());
                }
            }
        }
    }
    paths
        .into_iter()
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(from, to)| edges.get(from).unwrap().get(to).unwrap())
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from_input(input);
    let (_, edges) = grid.to_graph(false);
    let last_line = edges.get(&grid.end).unwrap();
    assert!(last_line.len() == 1);
    let target = last_line.keys().next().unwrap();
    let mut paths: Vec<Vec<Position>> = vec![];
    let mut q = VecDeque::from([vec![grid.start]]);
    while let Some(list) = q.pop_front() {
        if let Some(last) = list.last() {
            if last == target {
                paths.push(list.clone().into_iter().chain(vec![grid.end]).collect_vec());
            } else {
                for p in edges.get(last).unwrap().keys() {
                    if !list.contains(p) {
                        q.push_back(list.clone().into_iter().chain(vec![*p]).collect_vec());
                    }
                }
            }
        }
    }
    // println!("Path size: {}",paths.len());
    paths
        .into_iter()
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(from, to)| edges.get(from).unwrap().get(to).unwrap())
                .sum()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
