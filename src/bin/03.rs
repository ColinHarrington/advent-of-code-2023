use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = parse::schematic(input);
    let number_map: BTreeMap<(i32, i32), &NumberString> = map_numbers(&schematic.numbers);
    Some(
        schematic
            .symbols
            .into_keys()
            .flat_map(|spot| unique_neighbors(spot, &number_map))
            .unique()
            .map(|ns| ns.num_value())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = parse::schematic(input);
    let number_map: BTreeMap<(i32, i32), &NumberString> = map_numbers(&schematic.numbers);
    Some(
        schematic
            .symbols
            .into_keys()
            .map(|spot| unique_neighbors(spot, &number_map))
            .filter_map(|parts| match parts.len() {
                2 => Some(parts[0].num_value() * parts[1].num_value()),
                _ => None,
            })
            .sum(),
    )
}
fn map_numbers<'a>(
    numbers: &'a BTreeSet<NumberString<'a>>,
) -> BTreeMap<(i32, i32), &'a NumberString<'a>> {
    BTreeMap::from_iter(numbers.iter().flat_map(|ns| {
        (0..ns.value.len())
            .map(|n| n as i32)
            .map(|n| (ns.location.0, ns.location.1 + n))
            .map(move |spot| (spot, ns))
    }))
}
fn neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        /*(x,y),*/
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}
fn unique_neighbors<'a>(
    location: (i32, i32),
    number_map: &BTreeMap<(i32, i32), &'a NumberString<'a>>,
) -> Vec<&'a NumberString<'a>> {
    neighbors(location)
        .into_iter()
        .filter_map(|spot| number_map.get(&spot).copied())
        // .map(|ns| ns.clone())
        .unique()
        .collect()
}

#[derive(Debug)]
struct Schematic<'a> {
    numbers: BTreeSet<NumberString<'a>>,
    symbols: BTreeMap<(i32, i32), char>,
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone, Hash, Debug)]
struct NumberString<'a> {
    location: (i32, i32),
    value: &'a str,
}
impl NumberString<'_> {
    fn num_value(&self) -> u32 {
        self.value.parse().unwrap()
    }
}

mod parse {
    use crate::{NumberString, Schematic};
    use nom::branch::alt;
    use nom::character::complete;
    use nom::character::complete::{anychar, digit1, line_ending};
    use nom::combinator::{map, verify};
    use nom::multi::{many1, many1_count, separated_list1};
    use nom::IResult;
    use std::collections::{BTreeMap, BTreeSet};

    enum MapEntry<'a> {
        Spaces(usize),
        Digits(&'a str),
        Symbols(char),
    }

    pub fn schematic(input: &str) -> Schematic {
        let (_, rows) = rows(input).unwrap();

        let mut numbers: BTreeSet<NumberString> = BTreeSet::new();
        let mut symbols: BTreeMap<(i32, i32), char> = BTreeMap::new();

        for (row, entries) in rows.iter().enumerate() {
            let mut col = 0;
            for entry in entries {
                match entry {
                    MapEntry::Spaces(count) => col += count,
                    MapEntry::Digits(value) => {
                        numbers.insert(NumberString {
                            location: (row as i32, col as i32),
                            value,
                        });
                        col += value.len()
                    }
                    MapEntry::Symbols(c) => {
                        symbols.insert((row as i32, col as i32), *c);
                        col += 1;
                    }
                }
            }
        }
        Schematic { numbers, symbols }
    }
    // fn map_row(row:usize, Vec<MapEntry>)
    fn spaces(input: &str) -> IResult<&str, MapEntry> {
        map(many1_count(complete::char('.')), MapEntry::Spaces)(input)
    }

    fn digits(input: &str) -> IResult<&str, MapEntry> {
        map(digit1, MapEntry::Digits)(input)
    }
    fn symbol(input: &str) -> IResult<&str, MapEntry> {
        map(verify(anychar, is_symbol), MapEntry::Symbols)(input)
    }
    fn is_symbol(chr: &char) -> bool {
        match chr {
            '.' | '\r' | '\n' => false,
            c => !c.is_alphanumeric(),
        }
    }
    fn entry(input: &str) -> IResult<&str, MapEntry> {
        alt((spaces, digits, symbol))(input)
    }
    fn rows(input: &str) -> IResult<&str, Vec<Vec<MapEntry>>> {
        separated_list1(line_ending, many1(entry))(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
