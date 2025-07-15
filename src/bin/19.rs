use crate::parse::{parse, workflows};
use crate::Category::{A, M, S, X};
use crate::Comparator::{LessThan, MoreThan};
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (workflows, parts) = parse(input);
    let flows: HashMap<String, Workflow> = HashMap::from_iter(
        workflows
            .into_iter()
            .map(|workflow| (workflow.name.clone(), workflow)),
    );
    Some(
        parts
            .into_iter()
            .filter(|part| process(part, "in", &flows) == Destination::Accepted)
            .map(|part| part.total())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let flows: HashMap<String, Workflow> = HashMap::from_iter(
        workflows(input)
            .unwrap()
            .1
            .into_iter()
            .map(|workflow| (workflow.name.clone(), workflow)),
    );
    let mut queue = VecDeque::from([(
        Destination::Workflow("in".to_string()),
        RatingRange::new(1..=4000),
    )]);
    let mut accepted: Vec<RatingRange> = vec![];

    while let Some((dest, ranges)) = queue.pop_front() {
        if let Destination::Workflow(name) = dest {
            let workflow: &Workflow = flows.get(&name).unwrap();
            let mut last = Some(ranges.clone());
            for rule in workflow.rules.iter() {
                if let Some(ref range) = last {
                    match rule {
                        Rule::Compare(op, dest) => {
                            let (left, right) = op.split_ranges(range.clone());
                            if let Some(r) = left {
                                match dest {
                                    Destination::Accepted => {
                                        accepted.push(r);
                                    }
                                    _ => queue.push_back((dest.clone(), r)),
                                }
                            }
                            last = right;
                        }
                        Rule::Destination(dest) => match dest {
                            Destination::Workflow(_) => {
                                queue.push_back((dest.clone(), range.clone()))
                            }
                            Destination::Accepted => accepted.push(range.clone()),
                            Destination::Rejected => {}
                        },
                    }
                }
            }
        }
    }
    Some(accepted.iter().map(RatingRange::combinations).sum())
}

fn process(part: &Part, name: &str, workflows: &HashMap<String, Workflow>) -> Destination {
    match workflows.get(name).unwrap().evaluate(part) {
        Destination::Workflow(name) => process(part, &name, workflows),
        Destination::Accepted => Destination::Accepted,
        Destination::Rejected => Destination::Rejected,
    }
}

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}
impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => X,
            'm' => M,
            'a' => A,
            's' => S,
            _ => panic!("not a category"),
        }
    }
}

#[derive(Debug, Clone)]
enum Comparator {
    LessThan,
    MoreThan,
}
impl From<char> for Comparator {
    fn from(value: char) -> Self {
        match value {
            '>' => MoreThan,
            '<' => LessThan,
            _ => panic!("not a comparison"),
        }
    }
}
fn rating_range(start: usize, end: usize) -> Option<RangeInclusive<usize>> {
    match start <= end {
        true => Some(start..=end),
        false => None,
    }
}

impl Comparator {
    fn evaluate(&self, a: usize, b: usize) -> bool {
        match self {
            LessThan => a < b,
            MoreThan => a > b,
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<(&str, Vec<Rule>)> for Workflow {
    fn from((name, rules): (&str, Vec<Rule>)) -> Self {
        Workflow {
            name: name.to_string(),
            rules,
        }
    }
}
impl Workflow {
    fn evaluate(&self, part: &Part) -> Destination {
        self.rules
            .iter()
            .find_map(|rule| match rule {
                Rule::Compare(op, dest) => match op.evaluate(part) {
                    true => Some(dest.clone()),
                    false => None,
                },
                Rule::Destination(dest) => Some(dest.clone()),
            })
            .unwrap()
    }
}
#[derive(Debug, Clone)]
enum Rule {
    Compare(Comparison, Destination),
    Destination(Destination),
}
impl From<(Comparison, Destination)> for Rule {
    fn from((comparison, destination): (Comparison, Destination)) -> Self {
        Rule::Compare(comparison, destination)
    }
}
impl From<Destination> for Rule {
    fn from(destination: Destination) -> Self {
        Rule::Destination(destination)
    }
}

#[derive(Debug, Clone)]
struct Comparison {
    category: Category,
    comparator: Comparator,
    value: usize,
}
impl From<(Category, Comparator, u32)> for Comparison {
    fn from((category, comparator, value): (Category, Comparator, u32)) -> Self {
        Comparison {
            category,
            comparator,
            value: value as usize,
        }
    }
}
impl Comparison {
    fn evaluate(&self, part: &Part) -> bool {
        self.comparator
            .evaluate(part.rating(&self.category), self.value)
    }

    fn split_ranges(&self, ranges: RatingRange) -> (Option<RatingRange>, Option<RatingRange>) {
        let range = ranges.category_range(&self.category);
        let (left, right) = match self.comparator {
            LessThan => (
                rating_range(*range.start(), self.value - 1),
                rating_range(self.value, *range.end()),
            ),
            MoreThan => (
                rating_range(self.value + 1, *range.end()),
                rating_range(*range.start(), self.value),
            ),
        };

        (
            left.map(|left| ranges.update(&self.category, left)),
            right.map(|right| ranges.update(&self.category, right)),
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Destination {
    Workflow(String),
    Accepted,
    Rejected,
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        match value {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            _ => Destination::Workflow(value.to_string()),
        }
    }
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
impl Part {
    fn rating(&self, category: &Category) -> usize {
        match category {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }
    fn total(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct RatingRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl RatingRange {
    fn new(range: RangeInclusive<usize>) -> Self {
        RatingRange {
            x: range.clone(),
            m: range.clone(),
            a: range.clone(),
            s: range,
        }
    }

    fn category_range(&self, category: &Category) -> RangeInclusive<usize> {
        match category {
            X => self.x.clone(),
            M => self.m.clone(),
            A => self.a.clone(),
            S => self.s.clone(),
        }
    }

    fn update(&self, category: &Category, range: RangeInclusive<usize>) -> RatingRange {
        match category {
            X => RatingRange {
                x: range,
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            M => RatingRange {
                x: self.x.clone(),
                m: range,
                a: self.a.clone(),
                s: self.s.clone(),
            },
            A => RatingRange {
                x: self.x.clone(),
                m: self.m.clone(),
                a: range,
                s: self.s.clone(),
            },
            S => RatingRange {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: range,
            },
        }
    }

    fn combinations(&self) -> usize {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
    }
}
mod parse {
    use crate::{Category, Comparator, Comparison, Destination, Part, Rule, Workflow};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::character::complete::{alpha1, line_ending, one_of};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, separated_pair, tuple};
    use nom::IResult;

    pub fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
        let (tail, workflows) = workflows(input).unwrap();
        (workflows, parts(tail.trim()).unwrap().1)
    }
    pub fn workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
        separated_list1(line_ending, workflow)(input)
    }
    fn workflow(input: &str) -> IResult<&str, Workflow> {
        map(tuple((alpha1, rules)), Workflow::from)(input)
    }

    fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
        delimited(
            complete::char('{'),
            separated_list1(complete::char(','), rule),
            complete::char('}'),
        )(input)
    }
    fn rule(input: &str) -> IResult<&str, Rule> {
        alt((comparison_rule, destination_rule))(input)
    }
    fn destination_rule(input: &str) -> IResult<&str, Rule> {
        map(destination, Rule::from)(input)
    }
    fn comparison_rule(input: &str) -> IResult<&str, Rule> {
        map(
            separated_pair(comparison, complete::char(':'), destination),
            Rule::from,
        )(input)
    }

    fn destination(input: &str) -> IResult<&str, Destination> {
        map(alpha1, Destination::from)(input)
    }

    fn comparison(input: &str) -> IResult<&str, Comparison> {
        map(
            tuple((category, comparator, complete::u32)),
            Comparison::from,
        )(input)
    }
    fn comparator(input: &str) -> IResult<&str, Comparator> {
        map(one_of("<>"), Comparator::from)(input)
    }
    fn category(input: &str) -> IResult<&str, Category> {
        map(one_of("xmas"), Category::from)(input)
    }
    pub fn parts(input: &str) -> IResult<&str, Vec<Part>> {
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
