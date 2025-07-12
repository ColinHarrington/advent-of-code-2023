use crate::Category::{A, M, S, X};
use crate::Comparison::{LessThan, MoreThan};
use crate::parse::parts;

advent_of_code::solution!(19);


enum Category {
    X, M, A, S
}
impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => X,
            'm' => M,
            'a' => A,
            's'=> S,
            _ => panic!("not a category")
        }
    }
}

enum Comparison {
    LessThan, MoreThan
}
impl From<char> for Comparison {
    fn from(value: char) -> Self {
        match value {
            '>' => MoreThan,
            '<' => LessThan,
            _ => panic!("not a comparison")
        }
    }
}
struct Workflow {
    name: String,
    attr: char,
    comparison: char,
    left: String,
    right: String,
}

enum Foo {
    Workflow(String),
    Condition()
}

struct Condition {
    category: Category,
    comparison: Comparison,
    Left:

}
#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl From<(u64, u64, u64, u64)> for Part {
    fn from((x, m, a, s): (u64, u64, u64, u64)) -> Self {
        Part {
            x: x as usize,
            m: m as usize,
            a: a as usize,
            s: s as usize,
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let parts = parts(input.trim()).unwrap().1;
    println!("Parts: {:?}", parts);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

mod parse {
    use nom::character::complete;
    use crate::Part;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, one_of};
    use nom::combinator::map;
    use nom::multi::{separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, tuple};
    use nom::IResult;

    // fn workflows(input:&str) -> IResult<&str, Vec<&str>> {
    //     separated_list1(line_ending, many_till(line_ending))(input)
    // }
    // fn workflow(input: &str) -> IResult<&str, &str> {
    //     tuple((alpha1,tag("{"), one_of("xmas"), one_of("<>"), complete::u64, complete::char(':') tag("}")))(input)
    //     // take_until(complete::char('{}))(input)
    // }
    pub fn parts(input:&str) -> IResult<&str, Vec<Part>> {
        separated_list1(line_ending, part)(input)
    }
    fn part(input: &str) -> IResult<&str, Part> {
        map(
            delimited(
                complete::char('{'),
                tuple((
                    preceded(tag("x="), complete::u64),
                    preceded(tag(",m="), complete::u64),
                    preceded(tag(",a="), complete::u64),
                    preceded(tag(",s="), complete::u64),
                )),
                complete::char('}'),
            ),
            Part::from,
        )(input)
    }
    fn part_attribute(input: &str) -> IResult<&str, (char, u64)> {
        separated_pair(one_of("xmas"), complete::char('='), complete::u64)(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
