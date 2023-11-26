//! Day 21 - Monkey math

use std::collections::HashMap;

pub fn execute(op: u8, lhs: i64, rhs: i64) -> i64 {
    match op {
        b'+' => lhs + rhs,
        b'-' => lhs - rhs,
        b'*' => lhs * rhs,
        b'/' => lhs / rhs,
        x => panic!("Unknown operator {x}"),
    }
}

#[derive(Debug)]
pub enum Node {
    Number(i64),
    Math(u8, Box<Node>, Box<Node>),
    Human(i64),
}

impl Node {
    pub fn eval(&self) -> i64 {
        // In eval mode, "humn" is just another number shoutin' monkey
        match self {
            Node::Number(x) | Node::Human(x) => *x,
            Node::Math(op, lhs, rhs) => execute(*op, lhs.eval(), rhs.eval()),
        }
    }

    pub fn reduce(&self) -> Self {
        match self {
            Node::Number(x) => Node::Number(*x),
            Node::Human(x) => Node::Human(*x),
            Node::Math(op, lhs, rhs) => {
                let lhs = lhs.reduce();
                let rhs = rhs.reduce();
                match (&lhs, &rhs) {
                    (Node::Number(x), Node::Number(y)) => {
                        Node::Number(execute(*op, *x, *y))
                    },
                    _ => Node::Math(*op, Box::new(lhs), Box::new(rhs)),
                }
            }
        }
    }

    pub fn backtrack_from(&self, value: i64) -> i64 {
        match self {
            Node::Human(_) => {
                // Done, the cumulative value is the answer we're looking for
                value
            },
            Node::Math(op, lhs, rhs) => {
                let lhs = &**lhs;
                let rhs = &**rhs;
                match (lhs, rhs) {
                    (other, Node::Number(z)) => match op {
                        b'+' => other.backtrack_from(value - z),
                        b'-' => other.backtrack_from(value + z),
                        b'*' => other.backtrack_from(value / z),
                        b'/' => other.backtrack_from(value * z),
                        b'=' => other.backtrack_from(*z),
                        x => panic!("unknown operator {x}"),
                    },
                    (Node::Number(z), other) => match op {
                        b'+' => other.backtrack_from(value - z),
                        b'-' => other.backtrack_from(z - value),
                        b'*' => other.backtrack_from(value / z),
                        b'/' => other.backtrack_from(z / value),
                        b'=' => other.backtrack_from(*z),
                        x => panic!("unknown operator {x}"),
                    },
                    _ => {
                        panic!("Equation was not reduced first");
                    },
                }
            },
            _ => panic!("Stuck at an irreducible number"),
        }
    }

    pub fn build(name: &str, table: &HashMap<&str, Vec<&str>>) -> Self {
        let spec = &table[name];
        match spec.len() {
            1 => {
                let number = spec[0].parse().unwrap();
                if name == "humn" {
                    Self::Human(number)
                } else {
                    Self::Number(number)
                }
            },
            3 => {
                let op = spec[1].as_bytes()[0];
                let lhs = Self::build(spec[0], table);
                let rhs = Self::build(spec[2], table);

                Self::Math(op, Box::new(lhs), Box::new(rhs))
            },
            x => panic!("What kind of spec has {x} tokens?"),
        }
    }
}

pub fn load_monkeys<'a>(input: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    input
        .iter()
        .map(|line| line.split_once(':').unwrap())
        .map(|(name, rest)| (name, rest.split_whitespace().collect::<Vec<_>>()))
        .collect()
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 152; "with example data")]
    #[test_case(personal_input().as_slice() => 80326079210554; "with real data")]
    pub fn problem1(input: &[&str]) -> i64 {
        let map = load_monkeys(input);
        let root = Node::build("root", &map);
        root.eval()
    }

    #[test_case(SAMPLE_INPUT => 301; "with example data")]
    #[test_case(personal_input().as_slice() => 3617613952378; "with real data")]
    pub fn problem2(input: &[&str]) -> i64 {
        let mut map = load_monkeys(input);

        // Adjust the root node for the new operation
        let root = map.get_mut("root").unwrap();
        root[1] = "=";

        // Get the root node, reduce whichever branches we can from the
        // bottom up, and then solve it from the top down
        let root = Node::build("root", &map);
        let root = root.reduce();
        root.backtrack_from(0)
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
