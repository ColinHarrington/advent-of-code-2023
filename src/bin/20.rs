use crate::parse::parse;
use crate::Module::{Broadcaster, Conjunction, FlipFlop};
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let modules: HashMap<String, Module> =
        HashMap::from_iter(parse(input).into_iter().map(|module| match module.clone() {
            Broadcaster(output) => ("broadcaster".to_string(), Broadcaster(output)),
            FlipFlop(name, outputs) => (name.clone(), FlipFlop(name, outputs)),
            Conjunction(name, outputs) => (name.clone(), Conjunction(name, outputs)),
        }));
    let upstream: HashMap<String, Vec<String>> =
        modules.iter().fold(HashMap::new(), |mut m, (_, module)| {
            let (name, outputs) = match module {
                Broadcaster(outputs) => ("broadcaster".to_string(), outputs),
                FlipFlop(name, outputs) => (name.clone(), outputs),
                Conjunction(name, outputs) => (name.clone(), outputs),
            };
            for output in outputs {
                m.entry(output.to_string())
                    .or_default()
                    .push(name.to_string())
            }
            m
        });
    let conjunctions: HashMap<String, Vec<String>> = HashMap::from_iter(
        modules
            .iter()
            .filter(|(_, module)| matches!(module, Conjunction(_, _)))
            .map(|(name, _)| {
                (
                    name.clone(),
                    upstream
                        .get(name)
                        .unwrap()
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                )
            }),
    );

    let mut memory: HashMap<String, bool> =
        HashMap::from_iter(modules.iter().filter_map(|(_, module)| match module {
            FlipFlop(name, _) => Some((name.clone(), false)),
            Conjunction(name, _) => Some((name.clone(), false)),
            _ => None,
        }));
    let mut low_count = 0usize;
    let mut high_count = 0usize;
    let mut queue: VecDeque<(&str, Pulse)> = VecDeque::new();

    for _ in 0..1000 {
        queue.push_back(("broadcaster", Pulse::Low));
        low_count += 1;
        while let Some((name, incoming)) = queue.pop_front() {
            match modules.get(name) {
                Some(Broadcaster(outputs)) => {
                    for output in outputs {
                        queue.push_back((output, Pulse::Low));
                        low_count += 1;
                    }
                }
                Some(FlipFlop(name, outputs)) => match incoming {
                    Pulse::Low => {
                        let pulse = if memory[name] {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        memory
                            .entry(name.clone())
                            .and_modify(|v| *v = pulse == Pulse::High);
                        for output in outputs {
                            queue.push_back((output, pulse));
                            match pulse {
                                Pulse::High => high_count += 1,
                                Pulse::Low => low_count += 1,
                            }
                        }
                    }
                    Pulse::High => {}
                },
                Some(Conjunction(name, outputs)) => {
                    let pulse = match conjunctions
                        .get(name)
                        .unwrap()
                        .iter()
                        .map(|c| memory.get(c).unwrap())
                        .all(|v| *v)
                    {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };
                    memory
                        .entry(name.clone())
                        .and_modify(|v| *v = pulse == Pulse::High);
                    for output in outputs {
                        queue.push_back((output, pulse));
                        match pulse {
                            Pulse::High => high_count += 1,
                            Pulse::Low => low_count += 1,
                        }
                    }
                }
                None => {}
            }
        }
    }

    Some(low_count * high_count)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop(String, Vec<String>),
    Conjunction(String, Vec<String>),
}

mod parse {
    use crate::Module;
    use crate::Module::{Broadcaster, Conjunction, FlipFlop};
    use itertools::Itertools;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, line_ending};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    use nom::IResult;

    pub fn parse(input: &str) -> Vec<Module> {
        modules(input).unwrap().1
    }

    fn modules(input: &str) -> IResult<&str, Vec<Module>> {
        separated_list1(line_ending, module)(input)
    }

    fn module(input: &str) -> IResult<&str, Module> {
        alt((broadcaster, flip_flop, conjunction))(input)
    }

    fn conjunction(input: &str) -> IResult<&str, Module> {
        map(
            separated_pair(preceded(char('&'), alpha1), tag(" -> "), outputs),
            |(name, out)| Conjunction(name.to_string(), out),
        )(input)
    }

    fn flip_flop(input: &str) -> IResult<&str, Module> {
        map(
            separated_pair(preceded(char('%'), alpha1), tag(" -> "), outputs),
            |(name, out)| FlipFlop(name.to_string(), out),
        )(input)
    }

    fn broadcaster(input: &str) -> IResult<&str, Module> {
        map(preceded(tag("broadcaster -> "), outputs), |out| {
            Broadcaster(out)
        })(input)
    }
    fn outputs(input: &str) -> IResult<&str, Vec<String>> {
        map(separated_list1(tag(", "), alpha1), |out| {
            out.into_iter().map(|s: &str| s.to_string()).collect_vec()
        })(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example("20-2"));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
