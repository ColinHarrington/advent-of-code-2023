use crate::parse::sequences;
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Some(sequences(input).iter().map(|s| sequence_next(s)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(sequences(input).iter().map(|s| sequence_previous(s)).sum())
}

fn sequence_next(sequence: &[i64]) -> i64 {
    let delta = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    sequence.last().unwrap()
        + match delta.iter().all(|n| matches!(n, &0)) {
            true => 0,
            false => sequence_next(&delta),
        }
}

fn sequence_previous(sequence: &[i64]) -> i64 {
    let delta = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    sequence.first().unwrap()
        - match delta.iter().all(|n| matches!(n, &0)) {
            true => 0,
            false => sequence_previous(&delta),
        }
}

mod parse {
    use nom::character::complete;
    use nom::character::complete::line_ending;
    use nom::multi::separated_list1;
    use nom::IResult;

    pub fn sequences(input: &str) -> Vec<Vec<i64>> {
        separated_list1(line_ending, sequence)(input.trim())
            .unwrap()
            .1
    }
    fn sequence(input: &str) -> IResult<&str, Vec<i64>> {
        separated_list1(complete::char(' '), complete::i64)(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
