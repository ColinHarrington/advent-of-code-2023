use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

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
    let mut data = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    // let mut c = 0usize
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut rounds: Vec<Vec<Vec<char>>> = vec![];
    for c in 0..1000000000usize {
        data = cycle(data.clone());
        rounds.push(data.clone());
        if let Some(existing) = cache.insert(data.clone(), c) {
            let cycle_length = c - existing;
            let index = (1000000000usize - existing) % cycle_length + (existing - 1);
            println!("Cycle Length:{c}: {existing} :: {index}");
            return Some(score(rounds[index].clone()));
        }
    }

    Some(1)
}

/// Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east.
pub fn cycle(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    tilt_east(tilt_south(tilt_west(tilt_north(data))))
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
fn tilt_west(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = data.len();
    (0..size).map(|row| roll(data[row].clone())).collect_vec()
}

fn tilt_east(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = data.len();
    (0..size)
        .map(|row| {
            roll(data[row].clone().into_iter().rev().collect_vec())
                .into_iter()
                .rev()
                .collect_vec()
        })
        .collect_vec()
}
fn tilt_south(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = data.len();
    let columns = (0..size)
        .map(|col| roll((0..size).map(|row| data[row][col]).rev().collect_vec()))
        .collect_vec();

    (0..size)
        .map(|row| {
            (0..size)
                .map(|col| columns[col][size - 1 - row])
                .collect_vec()
        })
        .collect()
}

fn roll(line: Vec<char>) -> Vec<char> {
    line.split_inclusive(|ch| *ch == '#')
        .map(|group| group.iter().sorted_by(stone_sort).copied().collect_vec())
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
    #[test]
    fn test_cycle() {
        let mut data = to_data(&advent_of_code::template::read_file("examples", DAY));
        let cycles = [
            &advent_of_code::template::read_example(&format!("{}-cycle-1", DAY)),
            &advent_of_code::template::read_example(&format!("{}-cycle-2", DAY)),
            &advent_of_code::template::read_example(&format!("{}-cycle-3", DAY)),
        ];

        data = cycle(data.clone());
        assert_eq!(data, to_data(cycles[0]));

        data = cycle(data.clone());
        assert_eq!(data, to_data(cycles[1]));

        data = cycle(data.clone());
        assert_eq!(data, to_data(cycles[2]));
    }

    fn to_data(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec()
    }
}
