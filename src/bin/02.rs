// use crate::Color::{Blue, Green, Red};
use crate::parse::games;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = games(input).unwrap();

    let max = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    Some(
        games
            .iter()
            .filter(|game| game.possible(&max))
            .map(|game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, games) = games(input).unwrap();

    Some(
        games
            .iter()
            .map(|game| game.min_cubes())
            .map(|(r, g, b)| r * g * b)
            .sum(),
    )
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

type Turn = Vec<(u32, Color)>;
#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn possible(&self, max: &Cubes) -> bool {
        self.turns.iter().all(|dice| {
            dice.iter().all(|(count, color)| match color {
                Color::Red => max.red >= *count,
                Color::Green => max.green >= *count,
                Color::Blue => max.blue >= *count,
            })
        })
    }

    fn min_cubes(&self) -> (u32, u32, u32) {
        self.turns
            .iter()
            .map(|turn| turn.into())
            .fold(Cubes::min(), |acc, turn| acc.ceil(&turn))
            .into()
    }
}

type Rgb = (u32, u32, u32);

#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn min() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn ceil(&self, other: &Cubes) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl From<Rgb> for Cubes {
    fn from((red, green, blue): Rgb) -> Self {
        Self { red, green, blue }
    }
}

impl From<Cubes> for Rgb {
    fn from(val: Cubes) -> Self {
        (val.red, val.green, val.blue)
    }
}

impl From<&Turn> for Cubes {
    fn from(turn: &Turn) -> Self {
        turn.iter()
            .fold(
                (0u32, 0u32, 0u32),
                |(r, g, b), (count, color)| match color {
                    Color::Red => (*count, g, b),
                    Color::Green => (r, *count, b),
                    Color::Blue => (r, g, *count),
                },
            )
            .into()
    }
}

mod parse {
    use crate::{Color, Game};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, space1, u32 as u32_nom};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, tuple};
    use nom::IResult;
    use std::str::FromStr;

    pub fn games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(line_ending, game)(input)
    }
    fn game(input: &str) -> IResult<&str, Game> {
        map(
            tuple((preceded(tag("Game "), u32_nom), preceded(tag(": "), turns))),
            |(id, subsets)| Game { id, turns: subsets },
        )(input)
    }
    fn turns(input: &str) -> IResult<&str, Vec<Vec<(u32, Color)>>> {
        separated_list1(tag("; "), turn)(input)
    }
    fn turn(input: &str) -> IResult<&str, Vec<(u32, Color)>> {
        separated_list1(tag(", "), cube)(input)
    }
    fn cube(input: &str) -> IResult<&str, (u32, Color)> {
        separated_pair(u32_nom, space1, color)(input)
    }
    fn color(input: &str) -> IResult<&str, Color> {
        map(alt((tag("red"), tag("green"), tag("blue"))), |s| {
            Color::from_str(s).unwrap()
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
