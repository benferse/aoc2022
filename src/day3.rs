//! Day 3 - Rucksack organization

use std::collections::HashSet;

/// Given a string representing the entire contents of an elf's
/// ruck sack, partition that into the contents of each separate
/// compartment
///
/// # Examples
/// ```
/// use aoc2022::day3::*;
/// use std::collections::HashSet;
/// 
/// let (left, right) = partition_contents("ttgJtRGJQctTZtZT");
/// 
/// assert_eq!(left, HashSet::from(['t', 'g', 'J', 'R', 'G']));
/// assert_eq!(right, HashSet::from(['Q', 'c', 't', 'T', 'Z']));
/// ```
pub fn partition_compartments(all: &str) -> (HashSet<char>, HashSet<char>) {
    // str::len technically returns the length in bytes not
    // chars, but the input in known to be ASCII so /shrug
    let num = all.len() / 2;

    let left = HashSet::from_iter(all.chars().take(num));
    let right = HashSet::from_iter(all.chars().rev().take(num));

    (left, right)
}

const LOWERCASE_A_PRIORITY: u32 = 'a' as u32;
const UPPERCASE_A_PRIORITY: u32 = 'A' as u32;

/// Calculate the priority for a item
/// 
/// # Examples
/// ```
/// use aoc2022::day3::*;
/// assert_eq!(priority_for('a'), 1);
/// assert_eq!(priority_for('A'), 27);
/// ```
pub fn priority_for(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - LOWERCASE_A_PRIORITY + 1,
        'A'..='Z' => item as u32 - UPPERCASE_A_PRIORITY + 27,
        unknown => panic!("No idea what to do with this character {unknown}"),
    }
}

/// Given a set of rucksacks, figure out the total priority of the
/// items that are in the wrong compartments
/// 
/// # Examples
/// 
/// ```
/// use aoc2022::day3::*;
/// let rucksacks = [
///     "vJrwpWtwJgWrhcsFMMfFFhFp",
///     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
///     "PmmdzqPrVvPwwTWBwg",
///     "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
///     "ttgJtRGJQctTZtZT",
///     "CrZsJsPPZsGzwwsLwLmpwMDw",
/// ];
/// assert_eq!(prioritize_mistakes(&rucksacks), 157);
/// ```
pub fn prioritize_mistakes(rucksacks: &[&str]) -> u32
{
    rucksacks
        .iter()
        .map(|&x| partition_compartments(x))
        .flat_map(|(l, r)| HashSet::<_>::from_iter(l.intersection(&r).copied()))
        .map(priority_for)
        .sum()
}

/// Identify the badge (aka the single common element) across
/// a set of elves' rucksacks
/// 
/// # Examples
/// 
/// ```
/// use aoc2022::day3::*;
/// let rucksacks = [
///     "vJrwpWtwJgWrhcsFMMfFFhFp",
///     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
///     "PmmdzqPrVvPwwTWBwg",
/// ];
/// 
/// assert_eq!(identify_badge(&rucksacks), Some('r'));
pub fn identify_badge(rucksacks: &[&str]) -> Option<char> {
    let badges = rucksacks
        .iter()
        .map(|&contents| HashSet::<_>::from_iter(contents.chars()))
        .reduce(|accum, item| HashSet::from_iter(accum.intersection(&item).copied()));

    badges
        .iter()
        .flatten()
        .next()
        .cloned()
}

/// Given a set of rucksacks, identify the badges for each triplet
/// of elves and calculate the total priority across them
/// 
/// # Examples
/// ```
/// use aoc2022::day3::*;
/// let rucksacks = [
///     "vJrwpWtwJgWrhcsFMMfFFhFp",
///     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
///     "PmmdzqPrVvPwwTWBwg",
///     "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
///     "ttgJtRGJQctTZtZT",
///     "CrZsJsPPZsGzwwsLwLmpwMDw",
/// ];
/// assert_eq!(prioritize_badges(&rucksacks), 70);
/// ```
pub fn prioritize_badges(rucksacks: &[&str]) -> u32
{
    // Split the list into groups of three elves, find their badge,
    // and get the total priority
    rucksacks
        .chunks_exact(3)
        .flat_map(identify_badge)
        .map(priority_for)
        .sum()
}

#[cfg(test)]
mod answers {
    use super::*;
    use std::cell::LazyCell;

    const INPUT: LazyCell<Vec<&str>> = LazyCell::new(||
        include_str!("./input/day3.txt")
            .lines()
            .map(str::trim)
            .collect()
    );

    #[test]
    fn problem1() {
        assert_eq!(prioritize_mistakes(&INPUT), 8153);
    }

    #[test]
    fn problem2() {
        assert_eq!(prioritize_badges(&INPUT), 2342);
    }
}