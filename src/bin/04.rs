use crate::parse::load_cards;
use std::collections::BTreeSet;
use std::ops::Sub;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(load_cards(input).iter().map(|c| c.points()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards: Vec<usize> = load_cards(input)
        .iter()
        .map(|card| card.winners())
        .collect();

    Some(
        cards
            .iter()
            .enumerate()
            .fold(
                vec![1usize; cards.len()],
                |mut counts: Vec<usize>, (i, wins)| {
                    let my_count = counts[i];
                    for c in i..(i + wins) {
                        counts[c + 1] += my_count;
                    }
                    counts
                },
            )
            .into_iter()
            .sum(),
    )
}

#[derive(Debug)]
struct Card {
    winning: BTreeSet<u32>,
    entries: BTreeSet<u32>,
}
impl Card {
    fn winners(&self) -> usize {
        self.winning.intersection(&self.entries).count()
    }

    fn points(&self) -> u32 {
        match self.winners() {
            0 => 0,
            w => 2u32.pow(w.sub(1) as u32),
        }
    }
}

mod parse {
    use crate::Card;
    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::character::complete::{char, line_ending, multispace1};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
    use nom::IResult;
    use std::collections::BTreeSet;

    pub fn load_cards(input: &str) -> Vec<Card> {
        let (tail, cards) = cards(input.trim()).unwrap();
        assert_eq!("", tail);
        cards
    }
    fn card_number(input: &str) -> IResult<&str, usize> {
        map(
            preceded(
                tag("Card"),
                preceded(multispace1, terminated(complete::u32, char(':'))),
            ),
            |n| n as usize,
        )(input)
    }
    fn numbers(input: &str) -> IResult<&str, BTreeSet<u32>> {
        map(
            separated_list1(multispace1, complete::u32),
            |ns: Vec<u32>| BTreeSet::from_iter(ns),
        )(input)
    }
    fn card(input: &str) -> IResult<&str, Card> {
        map(
            tuple((
                terminated(card_number, multispace1),
                separated_pair(
                    numbers,
                    delimited(multispace1, char('|'), multispace1),
                    numbers,
                ),
            )),
            |(_, (winners, entries))| Card {
                winning: winners,
                entries,
            },
        )(input)
    }
    fn cards(input: &str) -> IResult<&str, Vec<Card>> {
        separated_list1(line_ending, card)(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
