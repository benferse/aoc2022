//! Day 25 - Full of hot air

pub fn from_snafu(input: &str) -> i64 {
    let mut value = 0;

    for (exp, mul) in input.chars().rev().enumerate() {
        let digit = match mul {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("bullshit"),
        };

        value += digit * 5i64.pow(exp as u32);
    }

    value
}

pub fn to_snafu(input: i64) -> String {
    if input == 0 {
        return String::new();
    }

    match input % 5 {
        rem @ 0..=2 => [to_snafu(input / 5), rem.to_string()].join(""),
        3 => [to_snafu(input / 5 + 1), String::from("=")].join(""),
        4 => [to_snafu(input / 5 + 1), String::from("-")].join(""),
        _ => panic!("what")
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => "2=-1=0"; "with sample data")]
    #[test_case(personal_input().as_slice() => "2-121-=10=200==2==21"; "with real data")]
    fn problem1(input: &[&str]) -> String {
        let answer = input
            .iter()
            .map(|&line| from_snafu(line))
            .sum();

        dbg!(answer);

        to_snafu(answer)
    }

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day25.txt")
            .lines()
            .collect()
    }

    const SAMPLE_INPUT: &[&str] = &[
        "1=-0-2",
        "12111",
        "2=0=",
        "21",
        "2=01",
        "111",
        "20012",
        "112",
        "1=-1=",
        "1-12",
        "12",
        "1=",
        "122",
    ];
}
