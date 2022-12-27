//! Day 24 - Blizzard Basin

use std::collections::HashSet;

pub fn parse_input(input: &[&str]) -> Vec<Vec<u8>> {
    let mut valley = vec![];

    for line in input {
        if !line.contains("###") {
            let row = line
                .as_bytes()
                .iter()
                .skip(1)
                .take(line.len() - 2)
                .copied()
                .collect();
            valley.push(row);
        }
    }

    valley
}

pub fn find_path(steps: i32, start: (i32, i32), end: (i32, i32), valley: &Vec<Vec<u8>>) -> i32 {
    let mut path_len: i32 = steps;
    let rows = valley.len() as i32;
    let cols = valley[0].len() as i32;
    let mut starting_positions = HashSet::from([start]);

    loop {
        let mut candidates = HashSet::<(i32, i32)>::new();
        for (row, col) in starting_positions.into_iter() {
            for (x, y) in [(row, col), (row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)] {
                if (x, y) == end {
                    return path_len;
                }
                if x >= 0 && x < rows && y >= 0 && y < cols &&
                    valley[x as usize][(y - path_len).rem_euclid(cols) as usize] != b'>' &&
                    valley[x as usize][(y + path_len).rem_euclid(cols) as usize] != b'<' &&
                    valley[(x - path_len).rem_euclid(rows) as usize][y as usize] != b'v' &&
                    valley[(x + path_len).rem_euclid(rows) as usize][y as usize] != b'^' {
                    candidates.insert((x, y));
                }
            }
        }

        starting_positions = candidates;
        if starting_positions.is_empty() {
            starting_positions.insert(start);
        }

        path_len += 1;
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 18; "with example data")]
    #[test_case(personal_input().as_slice() => 253; "with real data")]
    fn problem1(input: &[&str]) -> i32 {
        let valley = parse_input(input);
        let start = (-1, 0);
        let end = (valley.len() as i32, valley[0].len() as i32 - 1);
        find_path(1, start, end, &valley)
    }

    #[test_case(SAMPLE_INPUT => 54; "with example data")]
    #[test_case(personal_input().as_slice() => 794; "with real data")]
    fn problem2(input: &[&str]) -> i32 {
        let valley = parse_input(input);
        let start = (-1, 0);
        let end = (valley.len() as i32, valley[0].len() as i32 - 1);

        let trip1 = find_path(1, start, end, &valley);
        let trip2 = find_path(trip1, end, start, &valley);
        find_path(trip2, start, end, &valley)
    }

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day24.txt")
            .lines()
            .collect()
    }

    const SAMPLE_INPUT: &[&str] = &[
        "#.######",
        "#>>.<^<#",
        "#.<..<<#",
        "#>v.><>#",
        "#<^v^^>#",
        "######.#",
    ];
}
