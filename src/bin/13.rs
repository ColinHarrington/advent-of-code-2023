use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, reflection)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, smudgy_reflection)
}

fn solve(input: &str, reflection_finder: fn(Vec<String>) -> Vec<usize>) -> Option<usize> {
    let patterns = input
        .split("\n\n")
        .map(|p| p.lines().map(|s| s.to_string()).collect_vec())
        .collect_vec();
    let rotated_patterns = patterns
        .clone()
        .into_iter()
        .map(rotate_pattern)
        .collect_vec();

    let horizontal: usize = patterns.into_iter().flat_map(reflection_finder).sum();
    let vertical: usize = rotated_patterns
        .into_iter()
        .flat_map(reflection_finder)
        .sum();
    Some(horizontal * 100 + vertical)
}

fn rotate_pattern(pattern: Vec<String>) -> Vec<String> {
    let grid = pattern
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    (0..pattern.first().unwrap().len())
        .map(|column| grid.iter().map(|row| row[column]).join(""))
        .collect()
}
fn reflection(pattern: Vec<String>) -> Vec<usize> {
    (1..pattern.len())
        .filter(|&i| {
            pattern[0..i]
                .iter()
                .rev()
                .zip(pattern[i..].iter())
                .all(|(l, r)| l.eq(r))
        })
        .collect_vec()
}
fn smudgy_reflection(pattern: Vec<String>) -> Vec<usize> {
    (1..pattern.len())
        .filter(|&i| {
            pattern[0..i]
                .iter()
                .rev()
                .zip(pattern[i..].iter())
                .flat_map(|(l, r)| {
                    l.chars()
                        .zip(r.chars())
                        .map(|(a, b)| match a == b {
                            true => 0,
                            false => 1,
                        })
                        .collect_vec()
                })
                .sum::<usize>()
                == 1
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
