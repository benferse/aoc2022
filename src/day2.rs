//! Day 2 - Rock, Paper, Scissors

pub type Round = (char, char);

/// Calculate the score of a single round based on the
/// scoring rubric for problem 1
///
/// Examples
/// ```
/// use aoc2022::day2::*;
/// assert_eq!(problem_1_strat('A', 'Y'), 8);
/// ```
pub fn problem_1_strat(opponent_throw: char, your_throw: char) -> u32 {
    let shape_score = match your_throw {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        unknown => panic!("Unknown character encoding {unknown}"),
    };

    let outcome_score = match opponent_throw {
        'A' => match your_throw {
            'Y' => 6,
            'X' => 3,
            'Z' => 0,
            unknown => panic!("Unknown character encoding {unknown}"),
        },
        'B' => match your_throw {
            'Z' => 6,
            'Y' => 3,
            'X' => 0,
            unknown => panic!("Unknown character encoding {unknown}"),
        },
        'C' => match your_throw {
            'X' => 6,
            'Z' => 3,
            'Y' => 0,
            unknown => panic!("Unknown character encoding {unknown}"),
        },
        unknown => panic!("Unknown character encoding {unknown}"),
    };

    shape_score + outcome_score
}

/// Calculate the score of a single round based on the scoring
/// rubric for problem two, which means we interpret the second
/// directive not as a throw but as directions to proceed
///
/// # Examples
/// ```
/// use aoc2022::day2::*;
/// assert_eq!(problem_2_strat('A', 'Y'), 4)
/// ```
pub fn problem_2_strat(opponent_throw: char, your_throw: char) -> u32 {
    // "your_throw" is not what you should throw, but rather how
    // you should play.
    let your_actual_throw = match your_throw {
        // Rock - you need to lose
        'X' => match opponent_throw {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            unknown => panic!("Unknown character encoding {unknown}"),
        },
        // Paper - you need to draw
        'Y' => match opponent_throw {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            unknown => panic!("Unknown character encoding {unknown}"),
        },
        // Scissors - you need to win
        'Z' => match opponent_throw {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            unknown => panic!("Unknown character encoding {unknown}"),
        },
        // Huh?
        unknown => panic!("Unknown character encoding {unknown}"),
    };

    problem_1_strat(opponent_throw, your_actual_throw)
}

/// Given a set of rounds and a scoring rubric, determine the
/// total score for a full game of Rock/Paper/Scissors
///
/// # Examples
/// ```
/// use aoc2022::day2::*;
/// let rounds = [
///     ('A', 'Y'),
///     ('B', 'X'),
///     ('C', 'Z'),
/// ];
/// assert_eq!(score_game(rounds.iter(), problem_1_strat), 15);
/// ```
pub fn score_game<'a, R, F>(rounds: R, scoring: F) -> u32
where
    R: Iterator<Item = &'a Round>,
    F: Fn(char, char) -> u32,
{
    rounds.fold(0, |accum, item| accum + scoring(item.0, item.1))
}

pub fn parse_line(line: &str) -> Round {
    let mut chars = line.chars();
    let opponent_throw = chars.next().unwrap();
    let your_throw = chars.nth(1).unwrap();

    (opponent_throw, your_throw)
}

#[cfg(test)]
mod answers {
    use super::*;
    use std::cell::LazyCell;

    const INPUT: LazyCell<Vec<(char, char)>> = LazyCell::new(|| {
        include_str!("./input/day2.txt")
            .lines()
            .map(parse_line)
            .collect()
    });

    #[test]
    fn problem1() {
        assert_eq!(score_game(INPUT.iter(), problem_1_strat), 11063);
    }

    #[test]
    fn problem2() {
        assert_eq!(score_game(INPUT.iter(), problem_2_strat), 10349);
    }
}
