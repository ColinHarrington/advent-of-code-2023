use itertools::Itertools;
use num::Integer;
use std::collections::{HashSet, VecDeque};
use std::ops::Mul;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    Some(possibilities(
        64,
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(diamonds(
        26501365,
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    ))
}

fn possibilities(steps: usize, garden: Vec<Vec<char>>) -> usize {
    let width = garden.len();
    assert_eq!(garden.len(), garden[0].len()); //asserting square
    assert_eq!(width % 2, 1); // asserting odd width
    let midpoint = width / 2;
    assert_eq!(garden[midpoint][midpoint], 'S');

    let start = (midpoint, midpoint);
    let mut even = 0usize;
    let mut odd = 0usize;
    let mut queue = VecDeque::from([(0usize, midpoint, midpoint)]);
    let mut visisted = HashSet::from([start]);
    while let Some((step, row, col)) = queue.pop_front() {
        if step <= steps {
            if step.is_even() {
                even += 1;
            } else {
                odd += 1;
            }
            [
                (row + 1, col),
                (row, col + 1),
                (row.wrapping_sub(1), col),
                (row, col.wrapping_sub(1)),
            ]
            .into_iter()
            .filter(|(r, c)| *r < width && *c < width)
            .for_each(|(r, c)| {
                if visisted.insert((r, c)) && garden[r][c] == '.' {
                    queue.push_back((step + 1, r, c));
                }
            })
        }
    }
    if steps.is_even() {
        even
    } else {
        odd
    }
}

fn diamonds(steps: usize, garden: Vec<Vec<char>>) -> usize {
    // let width = garden.len();
    let cycles = steps / garden.len();
    assert_eq!(cycles % 2, 0);

    let (even, odd, corner) = diamond_count(garden);
    let even_diamonds = (1 + cycles.div_ceil(2).mul(2)).pow(2);
    let odd_diamonds = (cycles.div_ceil(2).mul(2)).pow(2);
    let corner_diamonds = (cycles * 2 + 1).pow(2) / 4;

    corner_diamonds * corner
        + match steps % 2 == 0 {
            true => even_diamonds * even + odd_diamonds * odd,
            false => even_diamonds * odd + odd_diamonds * even,
        }
}

fn diamond_count(garden: Vec<Vec<char>>) -> (usize, usize, usize) {
    let width = garden.len();
    assert_eq!(garden.len(), garden[0].len()); //asserting square
    assert_eq!(width % 2, 1); // asserting odd width
    let midpoint = width / 2;
    assert_eq!(garden[midpoint][midpoint], 'S');

    let start = (midpoint, midpoint);
    let mut even = 0usize;
    let mut odd = 0usize;
    let mut corner = 0usize;
    let mut queue = VecDeque::from([(0usize, midpoint, midpoint)]);
    let mut visisted = HashSet::from([start]);
    while let Some((step, row, col)) = queue.pop_front() {
        if step > midpoint {
            corner += 1;
        } else if step.is_even() {
            even += 1;
        } else {
            odd += 1;
        }
        [
            (row + 1, col),
            (row, col + 1),
            (row.wrapping_sub(1), col),
            (row, col.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|(r, c)| *r < width && *c < width)
        .for_each(|(r, c)| {
            if visisted.insert((r, c)) && garden[r][c] == '.' {
                queue.push_back((step + 1, r, c));
            }
        })
    }
    (even, odd, corner)
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
        let result = possibilities(6, garden);
        assert_eq!(result, 16);
    }
}
