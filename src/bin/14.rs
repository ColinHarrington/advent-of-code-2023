use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    Some(score(tilt_north(
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    )))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut platform = Platform {
        data: input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    };
    println!("{platform}");
    platform.cycle().cycle().cycle();
    None
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Platform {
    data: Vec<Vec<char>>,
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
                .map(|row| (0..size).map(|col| data[col][size - 1 - row]).collect_vec())
                .collect(),
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
                        'O' => Some(size - row),
                        _ => None,
                    })
            })
            .sum::<usize>()
    }

    /// Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east.
    fn cycle(&mut self) -> Platform {
        self.tilt_north();
        // println!("{self}");
        self.tilt_west();
        // println!("{self}");
        self.tilt_south();
        // println!("{self}");
        self.tilt_east();
        println!("{self}");
        self.clone()
    }
    fn rotate_right(&mut self) {
        let size = self.data.len();
        self.data = (0..size)
            .map(|row| {
                (0..size)
                    .map(|col| self.data[col][size - 1 - row])
                    .collect_vec()
            })
            .collect();
    }

    fn tilt_north(&mut self) {
        let size = self.data.len();

        let columns = (0..size)
            .map(|col| roll((0..size).map(|row| self.data[row][col]).collect_vec()))
            .collect_vec();
        columns.into_iter().enumerate().for_each(|(col, column)| {
            column
                .into_iter()
                .enumerate()
                .for_each(|(row, ch)| self.data[row][col] = ch)
        })
    }

    fn tilt_east(&mut self) {
        let size = self.data.len();
        let columns = (0..size)
            .map(|row| roll(self.data[row].clone().into_iter().rev().collect_vec()))
            .collect_vec();
        columns.into_iter().enumerate().for_each(|(row, column)| {
            column
                .into_iter()
                .enumerate()
                .for_each(|(col, ch)| self.data[row][size - 1 - col] = ch)
        })
    }

    fn tilt_south(&mut self) {
        let size = self.data.len();
        let columns = (0..size)
            .map(|col| {
                roll(
                    (0..size)
                        .map(|row| self.data[size - 1 - row][col])
                        .collect_vec(),
                )
            })
            .collect_vec();
        columns.into_iter().enumerate().for_each(|(col, column)| {
            column
                .into_iter()
                .enumerate()
                .for_each(|(row, ch)| self.data[size - 1 - row][col] = ch)
        })
    }
    fn tilt_west(&mut self) {
        let size = self.data.len();
        let columns = (0..size)
            .map(|row| roll(self.data[row].clone()))
            .collect_vec();
        columns.into_iter().enumerate().for_each(|(row, line)| {
            line.into_iter()
                .enumerate()
                .for_each(|(col, ch)| self.data[row][col] = ch)
        })
    }
}
impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .data
            .iter()
            .map(|row| row.iter().join(""))
            .collect_vec();
        writeln!(f, "{}\n", lines.join("\n"))
    }
}

fn tilt_north(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = data.len();
    let columns = (0..size)
        .map(|col| roll((0..size).map(|row| data[row][col]).collect_vec()))
        .collect_vec();

    (0..size)
        .map(|row| (0..size).map(|col| columns[col][row]).collect_vec())
        .collect()
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

    Platform { data }
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
fn score(data: Vec<Vec<char>>) -> usize {
    let size = data.len();
    data.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, ch)| match ch {
                    'O' => Some(size - row),
                    _ => None,
                })
        })
        .sum::<usize>()
}

// fn north_itr(size:usize) ->
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
