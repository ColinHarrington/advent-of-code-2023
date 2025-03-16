advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
                digits[0] * 10 + digits.iter().last().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.trim().lines().map(calibration_value).sum())
}

fn calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            'o' => read_digit(i, 3, line),
            't' | 's' => read_digit(i, 3, line).or(read_digit(i, 5, line)),
            'f' | 'n' => read_digit(i, 4, line),
            'e' => read_digit(i, 5, line),
            d => d.to_digit(10),
        })
        .collect();
    digits[0] * 10 + digits.iter().last().unwrap()
}

fn read_digit(index: usize, word_length: usize, line: &str) -> Option<u32> {
    line.get(index..index + word_length)
        .and_then(|word| match word {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example("01b"));
        assert_eq!(result, Some(281));
    }
}
