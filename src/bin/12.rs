use itertools::Itertools;
use std::cmp::Ordering;
// use std::cmp::Ordering::*;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(ConditionRecord::from)
            .map(|record| record.arrangements())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(unfold_condition_record)
            .map(ConditionRecord::from)
            .map(|record| record.arrangements())
            .sum(),
    )
}
fn unfold_condition_record(line: &str) -> String {
    let (s, g) = line.split_once(" ").unwrap();
    format!(
        "{} {}",
        [s, s, s, s, s].join("?"),
        [g, g, g, g, g].join(",")
    )
}

struct ConditionRecord {
    springs: Vec<u8>, // Space efficient
    groups: Vec<usize>,
}
impl From<&str> for ConditionRecord {
    fn from(line: &str) -> Self {
        let (springs, groups) = line.split_once(' ').unwrap();
        ConditionRecord {
            springs: springs.bytes().collect(),
            groups: groups
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec(),
        }
    }
}
impl From<String> for ConditionRecord {
    fn from(line: String) -> Self {
        ConditionRecord::from(line.as_str())
    }
}
impl ConditionRecord {
    fn arrangements(&self) -> usize {
        let mut dp = vec![vec![None; self.groups.len() + 1]; self.springs.len()];
        self.count(&mut dp, 0, 0)
    }

    fn count(&self, dp: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if i == self.springs.len() {
            return if j == self.groups.len() { 1 } else { 0 };
        }
        if let Some(v) = dp[i][j] {
            return v;
        }
        let res = match self.springs[i] {
            b'.' => self.count(dp, i + 1, j),
            b'#' => self.count_damaged(dp, i, j),
            b'?' => self.count(dp, i + 1, j) + self.count_damaged(dp, i, j),
            _ => panic!("invalid spring!"),
        };
        dp[i][j] = Some(res);
        res
    }

    fn count_damaged(&self, dp: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if j == self.groups.len() {
            return 0;
        }
        let group_end = i + self.groups[j];
        match group_end.cmp(&self.springs.len()) {
            Ordering::Greater => 0, // Not enough springs!
            Ordering::Equal => match self.springs[i..group_end].iter().contains(&b'.') {
                false if j == self.groups.len() - 1 => 1, // Exact amount and it matches
                _ => 0,
            },
            Ordering::Less => {
                match !self.springs[i..group_end].iter().contains(&b'.')  // No operational springs in group
                && self.springs[group_end] != b'#'
            {
                true => self.count(dp, group_end + 1, j + 1),
                false => 0,
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
