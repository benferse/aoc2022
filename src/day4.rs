//! Day 4 - Camp cleanup

use std::cmp::*;
use std::ops::RangeInclusive;

type Assignment = RangeInclusive<u8>;

#[derive(Copy, Clone, Debug, PartialEq, Eq,)]
pub enum OverlapType {
    /// Complete, one range complete contains another
    Full,

    /// Partial, the ranges partially overlap
    Partial,

    /// None, the ranges are fully disjoint
    None,
}

/// Given an assignment pair, determine how they overlap
///
/// # Examples
/// ```
/// use aoc2022::day4::*;
///
/// assert_eq!(get_overlap_type((1..=10, 5..=7)), OverlapType::Full);
/// assert_eq!(get_overlap_type((5..=7, 1..=10)), OverlapType::Full);
/// assert_eq!(get_overlap_type((5..=7, 6..=8)), OverlapType::Partial);
/// assert_eq!(get_overlap_type((3..=4, 5..=6)), OverlapType::None);
/// ```
pub fn get_overlap_type(assignments: (Assignment, Assignment)) -> OverlapType {
    let (lhs, rhs) = assignments;

    let overlapping_range = *min(lhs.start(), rhs.start())..=*max(lhs.end(), rhs.end());

    if overlapping_range.len() >= lhs.len() + rhs.len() {
        OverlapType::None
    } else if overlapping_range == lhs || overlapping_range == rhs {
        OverlapType::Full
    } else {
        OverlapType::Partial
    }
}

pub fn parse_line(line: &str) -> (Assignment, Assignment) {
    let (left, right) = line
        .split_once(',').expect("Ill-formed - no comma in an input line");

    (parse_assignment(left), parse_assignment(right))
}

pub fn parse_assignment(encoded: &str) -> Assignment {
    let (start, end) = encoded
        .split_once('-').expect("Ill-formed, no hyphen in assignment");

    start.parse().expect("Start value isn't a number")..=
    end.parse().expect("End value isn't a number")
}

/// Given a set of encoded assignment pairs, determine how many represent
/// overlaps of a particular type
///
/// # Examples
/// ```
/// use aoc2022::day4::*;
/// let samples = [
///     "2-4,6-8",
///     "2-3,4-5",
///     "5-7,7-9",
///     "2-8,3-7",
///     "6-6,4-6",
///     "2-6,4-8",
/// ];
/// assert_eq!(count_overlapping_assignments(&samples, false), 2);
/// assert_eq!(count_overlapping_assignments(&samples, true), 4);
pub fn count_overlapping_assignments(lines: &[&str], include_partial: bool) -> usize {
    lines
        .iter()
        .map(|&x| parse_line(x))
        .map(get_overlap_type)
        .fold(0, |accum, item| {
            if item == OverlapType::Full || (item == OverlapType::Partial && include_partial) {
                accum + 1
            } else {
                accum
            }
        })
}

#[cfg(test)]
mod answers {
    use super::*;
    use std::cell::LazyCell;

    const INPUT: LazyCell<Vec<&str>> = LazyCell::new(|| {
        include_str!("./input/day4.txt")
            .lines()
            .map(str::trim)
            .collect()
    });

    #[test]
    fn problem1() {
        assert_eq!(count_overlapping_assignments(&INPUT, false), 560);
    }

    #[test]
    fn problem2() {
        assert_eq!(count_overlapping_assignments(&INPUT, true), 839);
    }
}
