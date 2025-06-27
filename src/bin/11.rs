use itertools::Itertools;
use std::collections::BTreeSet;

advent_of_code::solution!(11);

type Galaxy = (usize, usize); // row, column
pub fn part_one(input: &str) -> Option<usize> {
    Some(galaxy_distances(input, 2))
}
pub fn part_two(input: &str) -> Option<usize> {
    Some(galaxy_distances(input, 100000))
}

pub fn galaxy_distances(input: &str, expansion: usize) -> usize {
    let galaxies: BTreeSet<Galaxy> = BTreeSet::from_iter(
        input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| line.chars().positions(is_galaxy).map(move |col| (row, col))),
    );
    let column_expansion = crate::expansion(
        BTreeSet::from_iter(galaxies.iter().map(|(_, col)| *col)),
        input.lines().next().unwrap().len(),
        expansion,
    );
    let row_expansion = crate::expansion(
        BTreeSet::from_iter(galaxies.iter().map(|(row, _)| *row)),
        input.lines().count(),
        expansion,
    );
    let expanded_galaxies = BTreeSet::from_iter(
        galaxies
            .into_iter()
            .map(|(row, col)| (row_expansion[row], column_expansion[col])),
    );

    expanded_galaxies
        .iter()
        .combinations(2)
        .map(|combo| (combo[0], combo[1]))
        .map(|(a, b)| distance(a, b))
        .sum::<usize>()
}
fn distance((ax, ay): &(usize, usize), (bx, by): &(usize, usize)) -> usize {
    ax.abs_diff(*bx) + ay.abs_diff(*by)
}

fn expansion(populated: BTreeSet<usize>, length: usize, size: usize) -> Vec<usize> {
    let mut offset = 0usize;
    (0..length)
        .map(|i| {
            if !populated.contains(&i) {
                offset += size - 1
            }
            i + offset
        })
        .collect_vec()
}

fn is_galaxy(ch: char) -> bool {
    ch == '#'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8200210));
    }
    #[test]
    fn test_part_two_10x() {
        let result = galaxy_distances(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);
    }
    #[test]
    fn test_part_two_100x() {
        let result = galaxy_distances(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, 8410);
    }
}
