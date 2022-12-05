//! Day 5 - Supply Stacks

use std::collections::VecDeque;
use std::str::FromStr;

pub struct Inventory {
    pub stacks: Vec<VecDeque<char>>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            stacks: vec![],
        }
    }

    pub fn reserve(&mut self, count: usize) {
        self.stacks = vec![VecDeque::new(); count];
    }

    pub fn add_inventory(&mut self, line: &str) {
        self.stacks.resize(line.len() / 4 + 1, VecDeque::new());

        for (idx, krate) in line
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .enumerate()
        {
            if krate[0] == '[' {
                self.stacks[idx].push_front(krate[1]);
            }
        }
    }

    pub fn execute_lifo(&mut self, direction: Direction) {
        for _ in 0..direction.count {
            if let Some(krate) = self.stacks[direction.from_idx - 1].pop_back() {
                self.stacks[direction.to_idx - 1].push_back(krate);
            }
        }
    }

    pub fn execute_fifo(&mut self, direction: Direction) {
        // Split last N off of the from stack and append them to the to stack
        let from = &mut self.stacks[direction.from_idx - 1];
        let mut to_move = from.split_off(from.len() - direction.count as usize);

        let to = &mut self.stacks[direction.to_idx - 1];
        to.append(&mut to_move);
    }

    pub fn iter(&self) -> impl Iterator<Item = &VecDeque<char>> {
        self.stacks.iter()
    }
}

pub struct Direction {
    pub count: u8,
    pub from_idx: usize,
    pub to_idx: usize,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s.split_whitespace().collect::<Vec<_>>();
        Ok(Self {
            count: pieces[1].parse().unwrap_or(0),
            from_idx: pieces[3].parse().unwrap_or(0),
            to_idx: pieces[5].parse().unwrap_or(0),
        })
    }
}

/// Given problem input, parse it out into stacks of crates
/// and directions for the crane
/// 
/// # Examples
/// ```
/// use aoc2022::day5::*;
///
/// let sample = r#"
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
/// 
/// move 1 from 2 to 3
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2 
/// "#;
/// 
/// let (inventory, directions) = parse_input(sample);
/// assert_eq!(inventory.stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
/// ```
pub fn parse_input(input: &str) -> (Inventory, Vec<Direction>) {
    let mut lines = input.lines();
    let mut stacks = Inventory::new();
    let mut directions = vec![];

    while let Some(line) = lines.next() {
        match line {
            crates if crates.trim_start().starts_with('[') => {
                stacks.add_inventory(crates);
            },
            direction if direction.starts_with('m') => {
                directions.push(direction.parse().unwrap());
            },
            _ => (),
        }
    }

    (stacks, directions)
} 

/// Given a set of crate stacks and a set of crane directions,
/// run the simulation to rearrange the crates
/// 
/// # Examples
/// ```
/// use aoc2022::day5::*;
/// let input = r#"
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3 
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2 
/// "#;
/// 
/// let (inventory, directions,) = parse_input(input);
/// let inventory = execute_instructions_9000(inventory, directions);
/// 
/// assert_eq!(inventory.stacks, vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']])
/// ```
pub fn execute_instructions_9000(mut stacks: Inventory, directions: Vec<Direction>) -> Inventory {
    for direction in directions {
        stacks.execute_lifo(direction);
    }

    stacks
}

/// Given a set of crate stacks and a set of crane directions,
/// run the simulation to rearrange the crates
/// 
/// # Examples
/// ```
/// use aoc2022::day5::*;
/// let input = r#"
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3 
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2 
/// "#;
/// 
/// let (inventory, directions,) = parse_input(input);
/// let inventory = execute_instructions_9001(inventory, directions);
/// 
/// assert_eq!(inventory.stacks, vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']])
/// ```
pub fn execute_instructions_9001(mut stacks: Inventory, directions: Vec<Direction>) -> Inventory {
    for direction in directions {
        stacks.execute_fifo(direction);
    }

    stacks
}

#[cfg(test)]
mod answers {
    use super::*;

    const INPUT: &str = include_str!("./input/day5.txt");

    #[test]
    pub fn problem1() {
        let (stacks, directions) = parse_input(&INPUT);
        let result = execute_instructions_9000(stacks, directions)
            .iter()
            .map(|v| v.back().unwrap())
            .collect::<String>()
            .clone();

        assert_eq!(&result, "SHMSDGZVC");
    }

    #[test]
    pub fn problem2() {
        let (stacks, directions) = parse_input(&INPUT);
        let result = execute_instructions_9001(stacks, directions)
            .iter()
            .map(|v| v.back().unwrap())
            .collect::<String>()
            .clone();

        assert_eq!(&result, "VRZGHDFBQ");
    }
}
