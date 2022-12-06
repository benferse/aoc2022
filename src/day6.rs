//! Day 6 - Tuning trouble

use core::hash::Hash;
use std::collections::HashSet;

trait FunOverEngineering {
    fn count_distinct(&self) -> usize;
}

impl<'a, T> FunOverEngineering for &'a &[T] where T: Hash + Eq {
    fn count_distinct(&self) -> usize {
        HashSet::<_>::from_iter(**self).len()
    }
}

/// ```
/// use aoc2022::day6::*;
///
/// assert_eq!(find_start_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
/// assert_eq!(find_start_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
/// assert_eq!(find_start_marker("aaaaaaa", 2), None);
/// ```
pub fn find_start_marker(signals: &str, len: usize) -> Option<usize> {
    signals
        .as_bytes()
        .windows(len)
        .enumerate()
        .find(|(_, packet)| packet.count_distinct() == packet.len())
        .map(|(idx, _)| idx + len)
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn problem1() {
        let input = include_str!("./input/day6.txt");
        assert_eq!(find_start_marker(input, 4), Some(1909));
    }

    #[test]
    fn problem2() {
        let input = include_str!("./input/day6.txt");
        assert_eq!(find_start_marker(input, 14), Some(3380));
    }
}
