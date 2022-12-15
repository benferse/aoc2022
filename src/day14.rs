//! Day 14 - Regolith Reservoir

use std::cmp::{min, max};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Unit {
    Rock,
    Sand,
}

pub trait PointExt {
    fn straight_down(&self) -> Self;
    fn down_left(&self) -> Self;
    fn down_right(&self) -> Self;
}

impl PointExt for (i32, i32) {
    fn straight_down(&self) -> Self {
        (self.0, self.1 + 1)
    }

    fn down_left(&self) -> Self {
        (self.0 - 1, self.1 + 1)
    }

    fn down_right(&self) -> Self {
        (self.0 + 1, self.1 + 1)
    }
}

pub fn simulate_falling_sand(mut cave: HashMap<(i32, i32), Unit>, abyss: i32) -> u32 {
    for x in 0.. {
        let mut sand = (500, 0);

        let at_rest = loop {
            if sand.1 > abyss {
                break sand;
            }

            let down = sand.straight_down();
            if !cave.contains_key(&down) {
                sand = down;
                continue;
            }

            let left = sand.down_left();
            if !cave.contains_key(&left) {
                sand = left;
                continue;
            }

            let right = sand.down_right();
            if !cave.contains_key(&right) {
                sand = right;
                continue;
            }

            break sand;
        };

        if at_rest.1 > abyss {
            return x;
        }

        cave.insert(at_rest, Unit::Sand);
    }

    unreachable!("wat")
}

pub fn simulate_with_floor(mut cave: HashMap<(i32, i32), Unit>, abyss: i32) -> u32 {
    for x in 1.. {
        let mut sand = (500, 0);

        let at_rest = loop {
            let down = sand.straight_down();
            if down.1 == abyss + 2 {
                break sand;
            } else if !cave.contains_key(&down) {
                sand = down;
                continue;
            }

            let left = sand.down_left();
            if left.1 == abyss + 2 {
                break sand;
            } else if !cave.contains_key(&left) {
                sand = left;
                continue;
            }

            let right = sand.down_right();
            if right.1 == abyss + 2 {
                break sand;
            } else if !cave.contains_key(&right) {
                sand = right;
                continue;
            }

            break sand;
        };

        if at_rest == (500, 0) {
            return x;
        }

        cave.insert(at_rest, Unit::Sand);
    }

    unreachable!("wat")
}

pub fn load_cave(input: &[&str]) -> (HashMap<(i32, i32), Unit>, i32) {
    // Each line is a single rock formation. Split the line into a set of
    // individual coordinates and load the rocks in
    let mut cave = HashMap::new();
    let mut abyss = 0;

    for line in input {
        for point in plot_points(line) {
            cave.insert(point, Unit::Rock);
            abyss = abyss.max(point.1);
        }
    }

    (cave, abyss)
}

pub fn plot_points(line: &str) -> impl Iterator<Item=(i32, i32)> {
    let tokens = line
        .split("->")
        .map(str::trim)
        .collect::<Vec<_>>();

    tokens
        .windows(2)
        .map(|window| {
            [to_point(window[0]), to_point(window[1])]
        })
        .fold(vec![], |mut accum, [j, k]| {
            if j.0 != k.0 {
                for q in min(j.0, k.0)..=max(j.0, k.0) {
                    accum.push((q, j.1));
                }
            } else {
                for q in min(j.1, k.1)..=max(j.1, k.1) {
                    accum.push((j.0, q));
                }
            }

            accum
        })
        .into_iter()
}

pub fn to_point(token: &str) -> (i32, i32) {
    let pieces = token
        .split_once(',')
        .expect("Expected number,number");

    (pieces.0.parse().unwrap(), pieces.1.parse().unwrap())
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 24; "with example data")]
    #[test_case(personal_input().as_slice() => 719; "with personal data")]
    fn problem1(input: &[&str]) -> u32 {
        let (cave, abyss) = load_cave(input);
        simulate_falling_sand(cave, abyss)
    }

    #[test_case(SAMPLE_INPUT => 93; "with example data")]
    #[test_case(personal_input().as_slice() => 23390; "with personal data")]
    fn problem2(input: &[&str]) -> u32 {
        let (cave, abyss) = load_cave(input);
        simulate_with_floor(cave, abyss)
    }

    const SAMPLE_INPUT: &[&str] = &[
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day14.txt")
            .lines()
            .collect()
    }
}
