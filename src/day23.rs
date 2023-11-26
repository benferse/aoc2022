//! Day 23 - Unstable diffusion

use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &[&str]) -> HashSet<(isize, isize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(col, char)| {
                    if char == '#' {
                        Some((row as isize, col as isize))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

pub fn diffuse(elves: &mut HashSet<(isize, isize)>, max_rounds: usize) -> usize {
    let mut move_order = vec![
        // N, NE, NW
        [(-1, 0), (-1, 1), (-1, -1)],
        // S, SE, SW
        [(1, 0), (1, 1), (1, -1)],
        // W, NW, SW
        [(0, -1), (-1, -1), (1, -1)],
        // E, NE, SE
        [(0, 1), (-1, 1), (1, 1)],
    ];

    let all_directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0),   (1, 1),
    ];

    for round in 0..max_rounds {
        let mut proposals = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();

        // Go through each elf and see if they would like to propose a move
        for elf in elves.iter() {
            // Do they want to move?
            if all_directions.iter().any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y))) {
                // Yup, they have a neighbor. Try to see if there are any valid moves
                for adjacent in &move_order {
                    if adjacent.iter().all(|(x, y)| !elves.contains(&(elf.0 + x, elf.1 + y))) {
                        // That direction is free - propose moving in the cardinal direction
                        let proposal = (elf.0 + adjacent[0].0, elf.1 + adjacent[0].1);
                        proposals
                            .entry(proposal)
                            .and_modify(|p| p.push(*elf))
                            .or_insert(vec![*elf]);
                        break;
                    }
                }
            }
        }

        // If noone wanted to move, we can end early
        if proposals.is_empty() {
            return round + 1;
        }

        // See which proposals can be honored
        for (proposal, applicants) in proposals {
            if applicants.len() == 1 {
                elves.remove(&applicants[0]);
                elves.insert(proposal);
            }
        }

        // Change the order in which directions are considered
        move_order.rotate_left(1);
    }

    0
}

pub fn calculate_empty_space(elves: &HashSet<(isize, isize)>) -> usize {
    let mut top_left = (isize::MAX, isize::MAX);
    let mut bot_right = (isize::MIN, isize::MIN);

    for elf in elves {
        top_left.0 = top_left.0.min(elf.0);
        top_left.1 = top_left.1.min(elf.1);

        bot_right.0 = bot_right.0.max(elf.0);
        bot_right.1 = bot_right.1.max(elf.1);
    }

    let width = (bot_right.0 - top_left.0).unsigned_abs() + 1;
    let height = (bot_right.1 - top_left.1).unsigned_abs() + 1;

    (width * height) - elves.len()
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SMALL_SAMPLE, 10 => 25; "with small sample data")]
    #[test_case(FULL_SAMPLE, 10 => 110; "with full sample data")]
    #[test_case(personal_input().as_slice(), 10 => 4034; "with personal data")]
    fn problem1(input: &[&str], rounds: usize) -> usize {
        let mut elves = parse_input(input);
        diffuse(&mut elves, rounds);
        calculate_empty_space(&elves)
    }

    #[test_case(FULL_SAMPLE, usize::MAX => 20; "with sample data")]
    #[test_case(personal_input().as_slice(), usize::MAX => 960; "with personal data")]
    fn problem2(input: &[&str], rounds: usize) -> usize {
        let mut elves = parse_input(input);
        diffuse(&mut elves, rounds)
    }

    const SMALL_SAMPLE: &[&str] = &[
        ".....",
        "..##.",
        "..#..",
        ".....",
        "..##.",
        ".....",
    ];

    const FULL_SAMPLE: &[&str] = &[
        "..............",
        "..............",
        ".......#......",
        ".....###.#....",
        "...#...#.#....",
        "....#...##....",
        "...#.###......",
        "...##.#.##....",
        "....#..#......",
        "..............",
        "..............",
        "..............",
    ];

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day23.txt")
            .lines()
            .map(str::trim)
            .collect()
    }
}
