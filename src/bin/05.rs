use crate::parse::read;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    read(input).seed_locations().into_iter().min()
}

pub fn part_two(input: &str) -> Option<i64> {
    let almanac: Almanac = read(input);
    almanac
        .translations()
        .into_iter()
        .fold(almanac.seed_ranges(), map_ranges)
        .first()
        .map(|l| l.start)
}

fn map_ranges(ranges: Vec<NumRange>, layer: Vec<Translation>) -> Vec<NumRange> {
    let mut mapped: Vec<NumRange> = vec![];
    let mut incoming: VecDeque<NumRange> = VecDeque::from(ranges);

    while let Some(range) = incoming.pop_front() {
        match layer.iter().find(|t| range.overlaps(&t.range)) {
            None => mapped.push(range),
            Some(t) => {
                if range.start < t.range.start {
                    mapped.push(NumRange {
                        start: range.start,
                        end: t.range.start - 1,
                    })
                }
                mapped.push(t.translate(&range));
                if range.end > t.range.end {
                    incoming.push_front(NumRange {
                        start: t.range.end + 1,
                        end: range.end,
                    })
                }
            }
        }
    }
    mapped
        .into_iter()
        .sorted()
        .collect_vec()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct NumRange {
    start: i64,
    end: i64,
}
impl NumRange {
    fn contains(&self, num: i64) -> bool {
        num >= self.start && num <= self.end
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}
impl Ord for NumRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}
impl PartialOrd for NumRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Translation {
    range: NumRange,
    delta: i64,
}
impl Translation {
    fn translate(&self, range: &NumRange) -> NumRange {
        NumRange {
            start: range.start.max(self.range.start) + self.delta,
            end: range.end.min(self.range.end) + self.delta,
        }
    }
}
impl From<Mapping> for Translation {
    fn from((dst, src, length): Mapping) -> Self {
        Translation {
            range: NumRange {
                start: src,
                end: src + length - 1,
            },
            delta: dst - src,
        }
    }
}

impl PartialOrd<Self> for Translation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Translation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

type Mapping = (i64, i64, i64);
type MappingLayer = Vec<Mapping>;
type MappingLayers = Vec<MappingLayer>;
#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    layers: MappingLayers,
}
impl Almanac {
    fn seed_locations(&self) -> Vec<i64> {
        let translations = self.translations();
        self.seeds
            .clone()
            .into_iter()
            .map(|seed| {
                translations.iter().fold(seed, |acc, layer| {
                    layer
                        .iter()
                        .find_map(|t| match t.range.contains(acc) {
                            true => Some(acc + t.delta),
                            false => None,
                        })
                        .unwrap_or(acc)
                })
            })
            .collect_vec()
    }
    fn seed_ranges(&self) -> Vec<NumRange> {
        self.seeds
            .iter()
            .tuples()
            .map(|(&start, length)| NumRange {
                start,
                end: start + length - 1,
            })
            .collect_vec()
    }
    fn translations(&self) -> Vec<Vec<Translation>> {
        self.layers
            .iter()
            .map(|layer| {
                layer
                    .clone()
                    .into_iter()
                    .map(Translation::from)
                    .sorted()
                    .collect()
            })
            .collect_vec()
    }
}
impl From<(Vec<i64>, MappingLayers)> for Almanac {
    fn from((seeds, layers): (Vec<i64>, MappingLayers)) -> Self {
        Self { seeds, layers }
    }
}
mod parse {
    use crate::{Almanac, Mapping};
    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::character::complete::{alpha1, char, line_ending};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, terminated, tuple};
    use nom::IResult;

    pub fn read(input: &str) -> Almanac {
        almanac(input.trim()).unwrap().1
    }

    fn almanac(input: &str) -> IResult<&str, Almanac> {
        map(
            tuple((
                terminated(seeds, tuple((line_ending, line_ending))),
                mappings,
            )),
            Almanac::from,
        )(input)
    }

    fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
        preceded(tag("seeds: "), separated_list1(char(' '), complete::i64))(input)
    }

    fn mapping(input: &str) -> IResult<&str, Mapping> {
        tuple((
            complete::i64,
            preceded(char(' '), complete::i64),
            preceded(char(' '), complete::i64),
        ))(input)
    }

    fn translation_map_label(input: &str) -> IResult<&str, (&str, &str)> {
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)
    }

    fn map_layer(input: &str) -> IResult<&str, Vec<Mapping>> {
        preceded(
            terminated(translation_map_label, line_ending),
            separated_list1(line_ending, mapping),
        )(input)
    }

    fn mappings(input: &str) -> IResult<&str, Vec<Vec<Mapping>>> {
        separated_list1(tuple((line_ending, line_ending)), map_layer)(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
