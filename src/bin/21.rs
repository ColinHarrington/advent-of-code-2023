use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(21);

fn possibilties(steps: usize, garden: Vec<Vec<char>>) -> usize {
    let width = garden.len();
    assert_eq!(garden.len(), garden[0].len()); //asserting square
    assert_eq!(width % 2, 1); // asserting odd width
    let midpoint = width / 2;
    assert_eq!(garden[midpoint][midpoint], 'S');

    let rocks: HashSet<(i32, i32)> =
        HashSet::from_iter(garden.iter().enumerate().flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, ch)| match ch {
                    '#' => Some((row as i32 - midpoint as i32, col as i32 - midpoint as i32)),
                    _ => None,
                })
        }));
    let result: HashSet<(i32, i32)> = (0..steps).fold(HashSet::from([(0, 0)]), |acc, _| {
        HashSet::from_iter(
            <HashSet<(i32, i32)> as IntoIterator>::into_iter(HashSet::from_iter(
                acc.into_iter().flat_map(neighbors),
            ))
            .filter(|point| !rocks.iter().contains(point)),
        )
    });
    result.len()
}

fn neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(possibilties(
        64,
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    ))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let garden = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let result = possibilties(6, garden);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
