use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::RangeInclusive;

advent_of_code::solution!(22);

#[derive(Debug, Clone)]
struct BrickStack {
    stack: BTreeSet<Brick>,
    over: Vec<Vec<usize>>,
    under: Vec<Vec<usize>>,
}
impl BrickStack {
    fn from_bricks(bricks: Vec<Brick>) -> BrickStack {
        let mut stack = BTreeSet::from_iter(bricks.clone());
        let len = bricks.len();
        let mut under: Vec<Vec<usize>> = vec![vec![]; len];
        let mut over: Vec<Vec<usize>> = vec![vec![]; len];
        for brick in bricks.iter().sorted_by(|a, b| a.z.start().cmp(b.z.start())) {
            let bounding_box = Brick {
                id: brick.id,
                x: brick.x.clone(),
                y: brick.y.clone(),
                z: 0..=(brick.z.start() - 1),
            };

            let supporting = stack
                .range(..=bounding_box.clone())
                .rev()
                .filter(|b| b.intersects(&bounding_box))
                .scan(None, |level, b| match level {
                    None => {
                        *level = Some(b.z.end());
                        Some(b.clone())
                    }
                    Some(l) if *l == b.z.end() => Some(b.clone()),
                    _ => None,
                })
                .collect_vec();

            let dropped = brick.drop_z(supporting.first().map(|b| *b.z.end()).unwrap_or(0) + 1);
            if *brick != dropped {
                stack.remove(brick);
                stack.insert(dropped.clone());
            }
            under[dropped.id] = supporting.iter().map(|b| b.id).collect_vec();
            for b in supporting {
                over[b.id].push(brick.id);
            }
        }
        BrickStack { stack, over, under }
    }

    fn safe_single_count(&self) -> usize {
        self.stack
            .iter()
            .filter(|b| self.over[b.id].iter().all(|a| self.under[*a].len() > 1))
            .count()
    }

    fn total_fall_count(&self) -> usize {
        self.stack
            .iter()
            .map(|brick| {
                let mut queue = VecDeque::from(vec![brick.id]);
                let mut fallen: HashSet<usize> = HashSet::from([brick.id]);
                while let Some(i) = queue.pop_front() {
                    for o in &self.over[i] {
                        if self.under[*o].iter().all(|b| fallen.contains(b)) {
                            queue.push_back(*o);
                            fallen.insert(*o);
                        }
                    }
                }
                fallen.len() - 1
            })
            .sum()
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        BrickStack::from_bricks(
            input
                .lines()
                .enumerate()
                .map(|(id, line)| Brick::from_line(line, id))
                .collect_vec(),
        )
        .safe_single_count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        BrickStack::from_bricks(
            input
                .lines()
                .enumerate()
                .map(|(id, line)| Brick::from_line(line, id))
                .collect_vec(),
        )
        .total_fall_count(),
    )
}

fn intersects(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Brick {
    id: usize,
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}
impl Brick {
    fn from_line(line: &str, id: usize) -> Self {
        let (left, right) = line.split_once('~').unwrap();
        let (x, y, z) = left
            .splitn(3, ',')
            .map(|n| n.parse::<u32>().unwrap())
            .zip(right.splitn(3, ',').map(|n| n.parse::<u32>().unwrap()))
            .map(|(a, b)| a.min(b)..=a.max(b))
            .collect_tuple()
            .unwrap();
        Brick { id, x, y, z }
    }
    fn intersects(&self, other: &Brick) -> bool {
        intersects(&self.x, &other.x)
            && intersects(&self.y, &other.y)
            && intersects(&self.z, &other.z)
    }
    fn drop_z(&self, z: u32) -> Self {
        Brick {
            id: self.id,
            x: self.x.clone(),
            y: self.y.clone(),
            z: RangeInclusive::new(z, z + self.z.end() - self.z.start()),
        }
    }
}

impl PartialOrd<Self> for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.z.end().cmp(other.z.end()) {
            Ordering::Equal => match self.x.end().cmp(other.x.end()) {
                Ordering::Equal => self.y.end().cmp(other.y.end()),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            },
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
