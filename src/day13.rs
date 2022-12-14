//! Day 13 - Distress signal

use serde_json::{json, Value};
use std::cmp::Ordering;

pub fn parse_input(lines: &[&str]) -> Vec<Value> {
    lines
        .iter()
        .filter_map(|&s| serde_json::from_str(s).ok())
        .collect()
}

pub fn compare(lhs: &Value, rhs: &Value) -> Ordering {
    match (lhs, rhs) {
        (Value::Number(a), Value::Number(b)) => {
            let a = a.as_u64().unwrap();
            let b = b.as_u64().unwrap();
            a.cmp(&b)
        },

        (Value::Array(a), Value::Array(b)) => {
            for x in 0..std::cmp::min(a.len(), b.len()) {
                let z = compare(&a[x], &b[x]);
                if z != Ordering::Equal {
                    return z;
                }
            }

            return a.len().cmp(&b.len());
        },

        (Value::Number(a), Value::Array(_)) => {
            compare(&json!(vec![a]), rhs)
        },

        (Value::Array(_), Value::Number(b)) => {
            compare(lhs, &json!(vec![b]))
        },

        _ => Ordering::Equal,
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 13; "with example data")]
    #[test_case(personal_input().as_slice() => 6187; "with personal data")]
    fn problem1(input: &[&str]) -> usize {
        parse_input(input)
            .chunks_exact(2)
            .enumerate()
            .filter(|(_, chunk)| matches!(compare(&chunk[0], &chunk[1]), Ordering::Less))
            .map(|(idx, _)| idx + 1)
            .sum()
    }

    #[test_case(SAMPLE_INPUT => 140; "with example data")]
    #[test_case(personal_input().as_slice() => 23520; "with personal data")]
    fn problem2(input: &[&str]) -> usize {
        let first_div = json!([[2]]);
        let second_div = json!([[6]]);
        let mut p: Vec<_> = parse_input(input);

        p.push(first_div);
        p.push(second_div);
        p.sort_by(|a, b| compare(a, b));

        let mut decoder = 1;

        for idx in 0..p.len() {
            if p[idx] == json!([[2]]) || p[idx] == json!([[6]]) {
                decoder *= idx + 1;
            }
        }
            
        decoder
    }

    const SAMPLE_INPUT: &[&str] = &[
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day13.txt")
            .lines()
            .collect()
    }
}
