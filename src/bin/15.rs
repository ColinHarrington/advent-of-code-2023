advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(|s| hash(s) as u32).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn hash(input: &str) -> u8 {
    input
        .chars()
        .map(|c| c as u8)
        .fold(0u8, |acc, c| acc.overflowing_add(c).0.overflowing_mul(17).0)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
