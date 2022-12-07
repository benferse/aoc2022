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
    use std::path::Path;
    use super::*;

    #[test]
    fn problems() {
        let input = include_str!("./input/day7.txt")
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>();

        let fs = build_filesystem(&input);

        // Part the first
        let hits: usize = fs
            .values()
            .filter(|&&v| v <= 100000)
            .sum();
        assert_eq!(hits, 1644735);

        // Part the second
        let total_free = 70000000 - *fs.get(Path::new("/")).unwrap_or(&0);
        let extra_required = 30000000 - total_free;

        let mut candidates = fs
            .values()
            .filter(|&&v| v >= extra_required)
            .collect::<Vec<_>>();
            
        candidates.sort_unstable();
        assert_eq!(candidates[0], &1300850);
    }
}
