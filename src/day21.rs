//! Day 21 - Monkey math

use std::collections::HashMap;

#[derive(Debug)]
pub struct Equation<'a> {
    lhs: &'a str,
    rhs: &'a str,
    op: u8,
}

pub type ShoutingMonkeys<'a> = HashMap<&'a str, i64>;
pub type ThinkingMonkeys<'a> = HashMap<&'a str, Equation<'a>>;

pub fn parse_input<'a>(input: &[&'a str]) -> (ShoutingMonkeys<'a>, ThinkingMonkeys<'a>) {
    let mut shouting_monkeys = HashMap::with_capacity(input.len());
    let mut thinking_monkeys = HashMap::with_capacity(input.len());

    for line in input {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let name = &tokens[0][0..tokens[0].len() - 1];
        match tokens.len() {
            2 => {
                let number = tokens[1].parse().unwrap();
                shouting_monkeys.insert(name, number);
            },

            4 => {
                let eq = Equation {
                    lhs: tokens[1],
                    rhs: tokens[3],
                    op: tokens[2].as_bytes()[0],
                };
                thinking_monkeys.insert(name, eq);
            },
            x => panic!("What kind of monkey is this with {x} tokens?"),
        }
    }

    (shouting_monkeys, thinking_monkeys)
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 152; "with example data")]
    #[test_case(personal_input().as_slice() => 80326079210554; "with personal data")]
    pub fn problem1(input: &[&str]) -> i64 {
        let (mut shouting, mut thinking) = parse_input(input);

        loop {
            let ready: Vec<_> = thinking
                .drain_filter(|_k, v| shouting.contains_key(v.lhs) && shouting.contains_key(v.rhs))
                .map(|(k, v)| {
                    let lhs = shouting.get(v.lhs).unwrap();
                    let rhs = shouting.get(v.rhs).unwrap();
                    let number = match v.op {
                        b'+' => lhs + rhs,
                        b'-' => lhs - rhs,
                        b'*' => lhs * rhs,
                        b'/' => lhs / rhs,
                        _ => 0,
                    };
                    (k, number)
                })
                .collect();

            shouting.extend(ready);

            if let Some(root_number) = shouting.get("root") {
                return *root_number;
            }
        }
    }

    #[test_case(SAMPLE_INPUT => 301; "with example data")]
    pub fn problem2(input: &[&str]) -> i64 {
        // Parse as before, but adjust root's operation
        let (mut shouting, mut thinking) = parse_input(input);
        thinking.get_mut("root").unwrap().op = b'=';
        0
    }

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day21.txt")
            .lines()
            .map(str::trim)
            .collect()
    }

    const SAMPLE_INPUT: &[&str] = &[
        "root: pppw + sjmn",
        "dbpl: 5",
        "cczh: sllz + lgvd",
        "zczc: 2",
        "ptdq: humn - dvpt",
        "dvpt: 3",
        "lfqf: 4",
        "humn: 5",
        "ljgn: 2",
        "sjmn: drzm * dbpl",
        "sllz: 4",
        "pppw: cczh / lfqf",
        "lgvd: ljgn * ptdq",
        "drzm: hmdt - zczc",
        "hmdt: 32",
    ];
}
