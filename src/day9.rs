//! Day 9 - Rope Bridge

use std::collections::HashSet;

pub type Instruction = (char, usize,);

/// # Examples
/// ```
/// use aoc2022::day9::*;
/// let sample = vec![
///     ('R', 4),
///     ('U', 4),
///     ('L', 3),
///     ('D', 1),
///     ('R', 4),
///     ('D', 1),
///     ('L', 5),
///     ('R', 2),
/// ];
///
/// assert_eq!(simulate_rope(2, sample.clone()).len(), 13);
/// assert_eq!(simulate_rope(10, sample.clone()).len(), 1);
///
/// let sample2 = vec![
///     ('R', 5),
///     ('U', 8),
///     ('L', 8),
///     ('D', 3),
///     ('R', 17),
///     ('D', 10),
///     ('L', 25),
///     ('U', 20),
/// ];
///
/// assert_eq!(simulate_rope(10, sample2).len(), 36);
///
/// ```
pub fn simulate_rope(num_knots: usize, instructions: Vec<Instruction>) -> HashSet<(i32, i32,)> {
    // All of the knots start piled up on each other.
    // The coordinate space doesn't really matter, so
    // let's say they start at (0, 0)
    let mut knots = vec![(0, 0); num_knots];

    // Tracking the tail knot, we know it visits at least the origin
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    // For every iteration
    // - Move the head knot n times in the right direction. It's the
    //   only knot known to move every step
    // - Each step, every knot that moves tugs on the knot behind
    //   it if they are no longer adjacent (if there is one)
    for instruction in instructions {
        let (direction, count) = instruction;

        for _ in 0..count {
            // Move the head of the rope
            match direction {
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                x => panic!("Bad direction {x}"),
            };

            // For each of the non-head knots, see if any movement
            // of the preceding knot should drag this one along
            for idx in 1..knots.len() {
                let prev = knots[idx-1];
                let curr = &mut knots[idx];

                let x_distance: i32 = prev.0 - curr.0;
                let y_distance: i32 = prev.1 - curr.1;

                if x_distance.abs() > 1 || y_distance.abs() > 1 {
                    if curr.0 != prev.0 && curr.1 != prev.1 {
                        // Pulling diagonally, so update both coords
                        curr.0 += x_distance.signum();
                        curr.1 += y_distance.signum();
                    } else if x_distance.abs() > 1 {
                        // Pulling horizontally
                        curr.0 += x_distance.signum();
                    } else {
                        // Pulling vertically
                        curr.1 += y_distance.signum();
                    }
                }
            }

            // At the end of every iteration, may as well record where the tail is
            visited.insert(knots[num_knots - 1]);
        }
    }
    
    visited
}


#[cfg(test)]
mod answers {
    use super::*;

    pub fn parse_instruction(input: &str) -> Instruction {
        let direction = input.chars().next().unwrap();
        let count = input.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();

        (direction, count)
    }

    #[test]
    fn problem1() {
        let input = include_str!("./input/day9.txt").lines().map(parse_instruction).collect();
        let results = simulate_rope(2, input);

        assert_eq!(results.len(), 6087);
    }

    #[test]
    fn problem2() {
        let input = include_str!("./input/day9.txt").lines().map(parse_instruction).collect();
        let results = simulate_rope(10, input);

        assert_eq!(results.len(), 2493);
    }
}
