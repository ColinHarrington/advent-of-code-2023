use crate::parse::parse;
use crate::Module::{Broadcaster, Conjunction, FlipFlop};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::ops::Sub;
use std::vec;

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

pub fn part_two(input: &str) -> Option<usize> {
    let nodes = module_map(parse(input));

    let mut conjunctions: HashMap<String, Option<usize>> = HashMap::from_iter(
        upstream_conjunctions("cs", &nodes)
            .into_iter()
            .skip(1)
            .map(|name| (name, None)),
    );

    let mut memory: HashMap<String, bool> =
        HashMap::from_iter(nodes.keys().map(|name| (name.to_string(), false)));

    let mut cycles: BTreeSet<usize> = BTreeSet::new();
    let mut iteration = 0usize;
    while !conjunctions.is_empty() {
        let mut queue: VecDeque<(&str, Pulse)> = VecDeque::new();
        queue.push_back(("broadcaster", Pulse::Low));
        while let Some((name, incoming)) = queue.pop_front() {
            match nodes.get(name) {
                None => {}
                Some(m) => match m.module_type {
                    ModuleType::Broadcaster => {
                        //(Some(Pulse::Low), &m.downstream)
                        for output in m.downstream.iter() {
                            queue.push_back((output, Pulse::Low));
                        }
                    }
                    ModuleType::Conjunction => {
                        let pulse = match m
                            .upstream
                            .iter()
                            .map(|c| memory.get(c).unwrap())
                            .all(|v| *v)
                        {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        if pulse == Pulse::Low {
                            match conjunctions.get(name) {
                                Some(None) => {
                                    conjunctions
                                        .entry(name.to_string())
                                        .and_modify(|value| *value = Some(iteration));
                                }
                                Some(Some(first)) => {
                                    cycles.insert(iteration.sub(*first));
                                    conjunctions.remove(name);
                                }
                                _ => {}
                            }
                        }
                        memory
                            .entry(name.to_string())
                            .and_modify(|v| *v = pulse == Pulse::High);
                        for output in m.downstream.iter() {
                            queue.push_back((output, pulse));
                        }
                    }
                    ModuleType::FlipFlop => match incoming {
                        Pulse::Low => {
                            let pulse = if memory[name] {
                                Pulse::Low
                            } else {
                                Pulse::High
                            };
                            memory
                                .entry(name.to_string())
                                .and_modify(|v| *v = pulse == Pulse::High);
                            for output in m.downstream.iter() {
                                queue.push_back((output, pulse));
                            }
                        }
                        Pulse::High => {}
                    },
                },
            }
        }
        iteration += 1;
    }
    // println!("cycles: {cycles:?}");
    Some(cycles.into_iter().reduce(lcm).unwrap_or(0))
}

/// https://www.hackertouch.com/least-common-multiple-in-rust.html
/// Lets not waste time reimplementing basics.
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
fn upstream_conjunctions(start: &str, modules: &HashMap<String, ModuleNode>) -> Vec<String> {
    let mut dependencies: BTreeSet<String> = BTreeSet::new();
    let mut upstream: Vec<String> = vec![];
    //Get node. recursively get all upstream from there that we haven't seen.
    let mut queue: VecDeque<String> = VecDeque::from([start.to_string()]);
    while let Some(current) = queue.pop_front() {
        // println!("  current: {current}");
        if let Some(node) = modules.get(&current) {
            // println!("    node: {node:?}");
            if node.module_type == ModuleType::Conjunction {
                upstream.push(current.clone());
            }
            for name in node.upstream.iter() {
                if dependencies.insert(name.to_string()) {
                    queue.push_back(name.to_string());
                }
            }
        }
    }
    // dependencies
    upstream
}

#[derive(Debug, Eq, PartialEq)]
enum ModuleType {
    Broadcaster,
    Conjunction,
    FlipFlop,
}

#[derive(Debug, Eq, PartialEq)]
struct ModuleNode {
    module_type: ModuleType,
    upstream: BTreeSet<String>,
    downstream: BTreeSet<String>,
}
fn module_map(modules: Vec<Module>) -> HashMap<String, ModuleNode> {
    let upstream_mapping: HashMap<String, Vec<String>> =
        modules.iter().fold(HashMap::new(), |mut m, module| {
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
    HashMap::from_iter(modules.into_iter().map(|m| {
        let upstream = BTreeSet::from_iter(
            upstream_mapping
                .get(&m.name())
                .unwrap_or(&vec![])
                .iter()
                .map(|n| n.to_string()),
        );
        (
            m.name(),
            ModuleNode {
                module_type: m.module_type(),
                upstream,
                downstream: m.outputs(),
            },
        )
    }))
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
impl Module {
    fn name(&self) -> String {
        match self {
            Broadcaster(_) => "broadcaster".to_string(),
            FlipFlop(name, _) => name.to_string(),
            Conjunction(name, _) => name.to_string(),
        }
    }
    fn module_type(&self) -> ModuleType {
        match self {
            Broadcaster(_) => ModuleType::Broadcaster,
            FlipFlop(_, _) => ModuleType::FlipFlop,
            Conjunction(_, _) => ModuleType::Conjunction,
        }
    }

    fn outputs(&self) -> BTreeSet<String> {
        match self {
            Broadcaster(outputs) => BTreeSet::from_iter(outputs.clone()),
            FlipFlop(_, outputs) => BTreeSet::from_iter(outputs.clone()),
            Conjunction(_, outputs) => BTreeSet::from_iter(outputs.clone()),
        }
    }
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
