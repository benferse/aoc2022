//! Day 2 - Rock, Paper, Scissors

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    pub fn shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn what_loses_to(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn what_beats(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn loses_to(&self, other: Throw) -> bool {
        other == self.what_beats()
    }

    pub fn beats(&self, other: Throw) -> bool {
        other == self.what_loses_to()
    }
}

pub type Round = (Throw, Throw);

/// Calculate the score of a single round based on the
/// scoring rubric for problem 1
///
/// Examples
/// ```
/// use aoc2022::day2::*;
/// assert_eq!(problem_1_strat(Throw::Rock, Throw::Paper), 8);
/// ```
pub fn problem_1_strat(opponent_throw: Throw, your_throw: Throw) -> u32 {
    // The first part of your score is fixed based on the shape
    // that you threw
    let shape_score = your_throw.shape_score();

    // The rest of the score is based on the outcome of the round
    let outcome_score = if your_throw.beats(opponent_throw) {
        6
    } else if your_throw.loses_to(opponent_throw) {
        0
    } else {
        3
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
/// assert_eq!(problem_2_strat(Throw::Rock, Throw::Paper), 4)
/// ```
pub fn problem_2_strat(opponent_throw: Throw, your_throw: Throw) -> u32 {
    // "your_throw" is not what you should throw, but rather how
    // you should play.
    let your_actual_throw = match your_throw {
        // Rock - you need to lose
        Throw::Rock => opponent_throw.what_loses_to(),
        // Paper - you need to draw
        Throw::Paper => opponent_throw,
        // Scissors - you need to win
        Throw::Scissors => opponent_throw.what_beats(),
    };

    // Pretend that's what you played all along
    problem_1_strat(opponent_throw, your_actual_throw)
}

/// Given a set of rounds and a scoring rubric, determine the
/// total score for a full game of Rock/Paper/Scissors
///
/// # Examples
/// ```
/// use aoc2022::day2::*;
/// let rounds = [
///     (Throw::Rock, Throw::Paper),
///     (Throw::Paper, Throw::Rock),
///     (Throw::Scissors, Throw::Scissors),
/// ];
/// assert_eq!(score_game(rounds.iter(), problem_1_strat), 15);
/// ```
pub fn score_game<'a, R, F>(rounds: R, scoring: F) -> u32
where
    R: Iterator<Item = &'a Round>,
    F: Fn(Throw, Throw) -> u32,
{
    rounds.fold(0, |accum, item| accum + scoring(item.0, item.1))
}

pub fn parse_line(line: &str) -> Round {
    let mut chars = line.chars();

    let opponent_throw = match chars.next() {
        Some('A') => Throw::Rock,
        Some('B') => Throw::Paper,
        Some('C') => Throw::Scissors,
        err => panic!("Input was not well formed: got {err:?} for an opponent's throw"),
    };

    let your_throw = match chars.nth(1) {
        Some('X') => Throw::Rock,
        Some('Y') => Throw::Paper,
        Some('Z') => Throw::Scissors,
        err => panic!("Input was not well formed: got {err:?} for our throw"),
    };

    (opponent_throw, your_throw)
}

#[cfg(test)]
mod answers {
    use super::*;
    use std::cell::LazyCell;

    const INPUT: LazyCell<Vec<Round>> = LazyCell::new(|| {
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
