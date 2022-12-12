//! Day 11 - Monkey business

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    quotient: u64,
    targets: (usize, usize),
    inspected: usize,
}

impl Monkey {
    pub fn do_operation(&self, worry_level: u64) -> u64 {
        (self.operation)(worry_level)
    }

    pub fn do_test(&self, worry_level: u64) -> usize {
        if worry_level % self.quotient == 0 {
            self.targets.0
        } else {
            self.targets.1
        }
    }
}

pub fn sample_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            operation: |x| x * 19,
            quotient: 23,
            targets: (2, 3),
            inspected: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |x| x + 6,
            quotient: 19,
            targets: (2, 0),
            inspected: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |x| x * x,
            quotient: 13,
            targets: (1, 3),
            inspected: 0,
        },
        Monkey {
            items: vec![74],
            operation: |x| x + 3,
            quotient: 17,
            targets: (0, 1),
            inspected: 0,
        }
    ]
}

pub fn personal_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![63, 57],
            operation: |x| x * 11,
            quotient: 7,
            targets: (6, 2),
            inspected: 0
        },
        Monkey {
            items: vec![82, 66, 87, 78, 77, 92, 83],
            operation: |x| x + 1,
            quotient: 11,
            targets: (5, 0),
            inspected: 0,
        },
        Monkey {
            items: vec![97, 53, 53, 85, 58, 54],
            operation: |x| x * 7,
            quotient: 13,
            targets: (4, 3),
            inspected: 0,
        },
        Monkey {
            items: vec![50],
            operation: |x| x + 3,
            quotient: 3,
            targets: (1, 7),
            inspected: 0,
        },
        Monkey {
            items: vec![64, 69, 52, 65, 73],
            operation: |x| x + 6,
            quotient: 17,
            targets: (3, 7),
            inspected: 0,
        },
        Monkey {
            items: vec![57, 91, 65],
            operation: |x| x + 5,
            quotient: 2,
            targets: (0, 6),
            inspected: 0,
        },
        Monkey {
            items: vec![67, 91, 84, 78, 60, 69, 99, 83],
            operation: |x| x * x,
            quotient: 5,
            targets: (2, 4),
            inspected: 0,
        },
        Monkey {
            items: vec![58, 78, 69, 65],
            operation: |x| x + 7,
            quotient: 19,
            targets: (5, 1),
            inspected: 0,
        }
    ]
}

pub fn simulate_round(monkeys: &mut Vec<Monkey>, relief: u64) {
    let num_monkeys = monkeys.len();

    // All the moduli are primes, so LCD is easy to figure out :)
    let subring_quotient: u64 = monkeys
        .iter()
        .map(|m| m.quotient)
        .product();

    for idx in 0..num_monkeys {
        let mut throws = vec![];

        for item in &monkeys[idx].items {
            let new_level = (monkeys[idx].do_operation(*item) / relief) % subring_quotient;
            let new_target = monkeys[idx].do_test(new_level);
            throws.push((new_level, new_target));
        }

        monkeys[idx].inspected += monkeys[idx].items.len();
        monkeys[idx].items.clear();

        for (x, y) in throws {
            monkeys[y].items.push(x);
        }
    }
}

pub fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut activity_levels = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();

    activity_levels.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    activity_levels[0] * activity_levels[1]
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(20, 3, sample_monkeys() => 10605; "sample data problem 1")]
    #[test_case(20, 3, personal_monkeys() => 107822; "real data problem 1")]
    #[test_case(10000, 1, sample_monkeys() => 2713310158; "sample data problem 2")]
    #[test_case(10000, 1, personal_monkeys() => 27267163742; "real data problem 2")]
    fn example_data(rounds: u32, relief: u64, mut monkeys: Vec<Monkey>) -> usize {
        for _ in 0..rounds {
            simulate_round(&mut monkeys, relief);
        }

        calculate_monkey_business(&monkeys)
    }
}
