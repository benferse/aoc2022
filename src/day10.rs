//! Day 10 - Cathode-ray tube

use std::collections::HashMap;

#[derive(Debug)]
pub enum Opcode {
    Noop,
    Addx(i32),
}

impl Opcode {
    fn parse(input: &str) -> Self {
        let tokens: Vec<&str> = input.splitn(2, ' ').collect();
        match tokens[0] {
            "noop" => Self::Noop,
            "addx" => Self::Addx(tokens[1].parse().unwrap()),
            instr => panic!("Unknown instruction {instr}"),
        }
    }
}

pub struct Cpu {
    x: i32,
    pipeline: HashMap<i32, i32>,
}

impl Cpu {
    pub fn init() -> Self {
        Self { x: 1, pipeline: Default::default() }
    }
}

pub fn exec(source_code: &[&str]) -> Vec<i32> {
    // Compile the source
    let mut compiled = source_code
        .iter()
        .map(|&x| Opcode::parse(x))
        .fuse();

    let mut cpu = Cpu::init();
    let mut signal_strengths = vec![];
    let mut crt = vec![];

    for cycle in 1.. {
        // During the cycle, emit the current signal strength if
        // it's time for that
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            signal_strengths.push(cycle * cpu.x);
        }

        // Update the CRT. The X register controls the current position
        // of the center of the sprite
        if i32::abs((cycle - 1) % 40 - cpu.x) <= 1 {
            crt.push('#');
        } else {
            crt.push('.');
        }

        // Cycle has finished. See if there are any pending instructions for this
        // cycle and execute them
        if let Some(pending) = cpu.pipeline.remove(&cycle) {
            cpu.x += pending;
        } else if let Some(next_instr) = compiled.next() {
            if let Opcode::Addx(pending) = next_instr {
                cpu.pipeline.insert(cycle + 1, pending);
            }
        } else if cpu.pipeline.is_empty() {
            println!("Stop after cycle {cycle}");
            break;
        }
    }

    for row in 0..6 {
        for col in 0..40 {
            let idx = row * 40 + col;
            print!("{}", crt[idx]);
        }
        println!();
    }

    signal_strengths
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn test_example_1() {
        let total: i32 = exec(EXAMPLE1)
            .iter()
            .sum();
        assert_eq!(total, 13140);
    }

    #[test]
    fn problem_1() {
        let input = include_str!("./input/day10.txt")
            .lines()
            .collect::<Vec<_>>();
        let total: i32 = exec(&input)
            .iter()
            .sum();
        assert_eq!(total, 12560);
    }

    const EXAMPLE1: &[&str] = &[
        "addx 15",
        "addx -11",
        "addx 6",
        "addx -3",
        "addx 5",
        "addx -1",
        "addx -8",
        "addx 13",
        "addx 4",
        "noop",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx -35",
        "addx 1",
        "addx 24",
        "addx -19",
        "addx 1",
        "addx 16",
        "addx -11",
        "noop",
        "noop",
        "addx 21",
        "addx -15",
        "noop",
        "noop",
        "addx -3",
        "addx 9",
        "addx 1",
        "addx -3",
        "addx 8",
        "addx 1",
        "addx 5",
        "noop",
        "noop",
        "noop",
        "noop",
        "noop",
        "addx -36",
        "noop",
        "addx 1",
        "addx 7",
        "noop",
        "noop",
        "noop",
        "addx 2",
        "addx 6",
        "noop",
        "noop",
        "noop",
        "noop",
        "noop",
        "addx 1",
        "noop",
        "noop",
        "addx 7",
        "addx 1",
        "noop",
        "addx -13",
        "addx 13",
        "addx 7",
        "noop",
        "addx 1",
        "addx -33",
        "noop",
        "noop",
        "noop",
        "addx 2",
        "noop",
        "noop",
        "noop",
        "addx 8",
        "noop",
        "addx -1",
        "addx 2",
        "addx 1",
        "noop",
        "addx 17",
        "addx -9",
        "addx 1",
        "addx 1",
        "addx -3",
        "addx 11",
        "noop",
        "noop",
        "addx 1",
        "noop",
        "addx 1",
        "noop",
        "noop",
        "addx -13",
        "addx -19",
        "addx 1",
        "addx 3",
        "addx 26",
        "addx -30",
        "addx 12",
        "addx -1",
        "addx 3",
        "addx 1",
        "noop",
        "noop",
        "noop",
        "addx -9",
        "addx 18",
        "addx 1",
        "addx 2",
        "noop",
        "noop",
        "addx 9",
        "noop",
        "noop",
        "noop",
        "addx -1",
        "addx 2",
        "addx -37",
        "addx 1",
        "addx 3",
        "noop",
        "addx 15",
        "addx -21",
        "addx 22",
        "addx -6",
        "addx 1",
        "noop",
        "addx 2",
        "addx 1",
        "noop",
        "addx -10",
        "noop",
        "noop",
        "addx 20",
        "addx 1",
        "addx 2",
        "addx 2",
        "addx -6",
        "addx -11",
        "noop",
        "noop",
        "noop",
    ];
}
