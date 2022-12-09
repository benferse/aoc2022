//! Day 7 - No space left on device

use std::collections::HashMap;
use std::path::PathBuf;

pub fn build_filesystem(terminal_output: &[&str]) -> HashMap<PathBuf, usize> {
    let mut current_path = PathBuf::from("/");
    let mut files = HashMap::new();

    for line in terminal_output {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("$") => {
                match tokens.next() {
                    Some("ls") => (),
                    Some("cd") => {
                        match tokens.next() {
                            Some("/") =>  { current_path = PathBuf::from("/"); },
                            Some("..") => { current_path.pop(); },
                            Some(dir) =>  { current_path.push(dir); },
                            _ => panic!("malformed cd command"),
                        };
                    },
                    command => panic!("Unrecognized input {command:?}"),
                }
            },
            Some("dir") => (),
            Some(size) => {
                let size: usize = size.parse().unwrap();
                for ancestor in current_path.ancestors() {
                    *files
                        .entry(ancestor.to_owned())
                        .or_insert(0) += size;
                }

            },
            unknown => panic!("Unrecognized input {unknown:?}"),
        }
    }

    files
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn problems() {
        let input = include_str!("./input/day7.txt")
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>();

        let mut dir_sizes = build_filesystem(&input)
            .values()
            .copied()
            .collect::<Vec<_>>();
        dir_sizes.sort_unstable();

        // Part the first
        let little_guys: usize = dir_sizes
            .iter()
            .take_while(|&&v| v <= 100000)
            .sum();
        assert_eq!(little_guys, 1644735);

        // Part the second
        let extra_required = dir_sizes.last().unwrap_or(&0) - 40000000;
        let smallest = dir_sizes
            .iter()
            .find(|&&v| v >= extra_required);

        assert_eq!(smallest, Some(&1300850));
    }
}
