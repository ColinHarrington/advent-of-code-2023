use crate::parse::read;
use crate::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};
use itertools::Itertools;
use std::cmp::Ordering;
use std::panic;
use std::str::FromStr;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        read(input)
            .into_iter()
            .map(camel_card_jacks)
            .sorted()
            .enumerate()
            .map(ranked_score)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        read(input)
            .into_iter()
            .map(camel_card_jokers)
            .sorted()
            .enumerate()
            .map(ranked_score)
            .sum(),
    )
}
type CharCards = [char; 5];
type Hand = [Card; 5];

fn ranked_score((rank, card): (usize, CamelCard)) -> u64 {
    (rank + 1) as u64 * card.bid
}
fn camel_card_jacks((chars, bid): (CharCards, u64)) -> CamelCard {
    CamelCard::from((parse_cards(chars, Card::Jack), bid))
}
fn camel_card_jokers((chars, bid): (CharCards, u64)) -> CamelCard {
    CamelCard::from((parse_cards(chars, Card::Joker), bid))
}
fn parse_cards(chars: CharCards, jay: Card) -> Hand {
    Hand::from(
        <[Card; 5]>::try_from(
            chars
                .into_iter()
                .map(|c| match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => jay,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("Invalid Card"),
                })
                .collect_vec(),
        )
        .unwrap(),
    )
}
fn rate_hand(hand: &Hand) -> HandType {
    let remaining: Vec<usize> = hand
        .iter()
        .counts()
        .into_iter()
        .filter_map(|(&card, count)| match card {
            Card::Joker => None,
            _ => Some(count),
        })
        .sorted()
        .rev()
        .collect_vec();
    match remaining.len() {
        0 | 1 => FiveKind,
        2 => HandType::from((remaining[0], remaining[1])),
        3 => HandType::from((remaining[0], remaining[1], remaining[2])),
        4 => OnePair,
        5 => HighCard,
        _ => panic!("Invalid Hand"),
    }
}
#[derive(Debug, Eq, PartialEq)]
struct CamelCard {
    hand: Hand,
    hand_type: HandType,
    bid: u64,
}
impl From<(Hand, u64)> for CamelCard {
    fn from((hand, bid): (Hand, u64)) -> Self {
        Self {
            hand,
            hand_type: rate_hand(&hand),
            bid,
        }
    }
}
impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .hand
                .iter()
                .zip(other.hand.iter())
                .find_map(|(c1, c2)| match c1.cmp(c2) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Ordering::Greater),
                })
                .unwrap_or(Ordering::Equal),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        Card::from_str(&value.to_string()).unwrap()
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(format!("Invalid card {s}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
impl From<(usize, usize)> for HandType {
    fn from(remaining: (usize, usize)) -> Self {
        match remaining {
            (4, 1) => FourKind,
            (3, 2) => FullHouse,
            (3, 1) => FourKind,
            (2, 2) => FullHouse,
            (2, 1) => FourKind,
            (1, 1) => FourKind,
            _ => panic!("Invalid Hand"),
        }
    }
}
impl From<(usize, usize, usize)> for HandType {
    fn from(remaining: (usize, usize, usize)) -> Self {
        match remaining {
            (3, 1, 1) => ThreeKind,
            (2, 2, 1) => TwoPair,
            (2, 1, 1) => ThreeKind,
            (1, 1, 1) => ThreeKind,
            _ => panic!("Invalid Hand"),
        }
    }
}
mod parse {
    use crate::CharCards;
    use nom::character::complete::{char, line_ending, one_of, u64 as nom_u64};
    use nom::combinator::map;
    use nom::multi::{count, separated_list1};
    use nom::sequence::separated_pair;
    use nom::IResult;

    pub fn read(input: &str) -> Vec<(CharCards, u64)> {
        let (tail, camel_cards) = camel_cards(input.trim()).unwrap();
        assert_eq!("", tail);
        camel_cards
    }
    fn camel_cards(input: &str) -> IResult<&str, Vec<(CharCards, u64)>> {
        separated_list1(line_ending, camel_card)(input)
    }
    fn camel_card(input: &str) -> IResult<&str, (CharCards, u64)> {
        separated_pair(cards, char(' '), nom_u64)(input)
    }

    fn cards(input: &str) -> IResult<&str, CharCards> {
        map(count(one_of("AKQJT98765432"), 5), |chars| {
            CharCards::try_from(chars).unwrap()
        })(input)
    }
}
#[cfg(test)]
mod tests {
    use crate::Card::{
        Ace, Eight, Five, Four, Jack, Joker, King, Nine, Queen, Seven, Six, Ten, Three, Two,
    };
    use crate::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};
    use crate::{part_one, part_two, DAY};

    #[test]
    fn test_card_order() {
        assert!(Ace > King);
        assert!(Ace > Two);
        assert_eq!(Ace, Ace);

        for card in vec![Ace, King, Queen, Jack, Ten] {
            assert!(Nine < card)
        }
        assert_eq!(Nine, Nine);
        for card in vec![Eight, Seven, Six, Five, Four, Three, Two, Joker] {
            assert!(Nine > card)
        }
    }

    #[test]
    fn test_hand_ranking() {
        assert!(FiveKind > FourKind);
        assert!(FourKind > FullHouse);
        assert!(FullHouse > ThreeKind);
        assert!(ThreeKind > TwoPair);
        assert!(TwoPair > OnePair);
        assert!(OnePair > HighCard);
        assert_eq!(HighCard, HighCard);

        assert!(ThreeKind > OnePair);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
