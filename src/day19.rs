//! Day 19 - Not enough minerals

use rand::prelude::*;

pub type Robots = [u32; 5];
pub type Resources = [u32; 5];

const NONE: usize = 0;
const ORE: usize = 1;
const CLAY: usize = 2;
const OBSIDIAN: usize = 3;
const GEODE: usize = 4;

#[derive(Debug)]
pub struct Blueprint {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: (u32, u32),
    geode_cost: (u32, u32),
}

impl Blueprint {
    pub fn from_str(line: &str) -> Self {
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        Self {
            ore_cost: tokens[6].parse().unwrap(),
            clay_cost: tokens[12].parse().unwrap(),
            obsidian_cost: (tokens[18].parse().unwrap(), tokens[21].parse().unwrap()),
            geode_cost: (tokens[27].parse().unwrap(), tokens[30].parse().unwrap()),
        }
    }

    pub fn select_next(&self, resources: &mut Resources, rng: &mut ThreadRng, _: u32) -> Option<usize> {
        // figure out what we could build right now, and pick one at random (:
        let mut buildable = vec![NONE];

        if resources[ORE] >= self.geode_cost.0 && resources[OBSIDIAN] >= self.geode_cost.1 {
            buildable.push(GEODE);
        } 

        if resources[ORE] >= self.obsidian_cost.0 && resources[CLAY] >= self.obsidian_cost.1 {
            buildable.push(OBSIDIAN);
        } 

        if resources[ORE] >= self.clay_cost {
            buildable.push(CLAY);
        } 

        if resources[ORE] >= self.ore_cost {
            buildable.push(ORE);
        }

        match buildable.choose_weighted(rng, |elem| (*elem + 1) * (2)) {
            Ok(&ORE) => {
                resources[ORE] -= self.ore_cost;
                Some(ORE)
            },
            Ok(&CLAY) => {
                resources[ORE] -= self.clay_cost;
                Some(CLAY)
            },
            Ok(&OBSIDIAN) => {
                resources[ORE] -= self.obsidian_cost.0;
                resources[CLAY] -= self.obsidian_cost.1;
                Some(OBSIDIAN)
            },
            Ok(&GEODE) => {
                resources[ORE] -= self.geode_cost.0;
                resources[OBSIDIAN] -= self.geode_cost.1;
                Some(GEODE)
            },
            _ => {
                None
            },
        }
    }
}

pub fn simulate(blueprint: &Blueprint, runtime: u32) -> usize {
    // Run n minutes of the simulation. Each minute involves three
    // phases
    // - Spend: optionally spend resources to start building a new robot
    // - Produce: each complete robot produces one of its resources
    // - Build: the build started in the spend phase completes and a new robot is born
    let mut resources: Resources = [0, 0, 0, 0, 0];
    let mut robots: Robots = [0, 1, 0, 0, 0];
    let mut rng = thread_rng();

    for n in 0..runtime {
        // Choose to spend
        let in_progress = blueprint.select_next(&mut resources, &mut rng, n);

        // Each robot producesgg
        resources[ORE] += robots[ORE];
        resources[CLAY] += robots[CLAY];
        resources[OBSIDIAN] += robots[OBSIDIAN];
        resources[GEODE] += robots[GEODE];

        // Finish building
        if let Some(new_robot) = in_progress {
            robots[new_robot] += 1;
        }
    }

    resources[GEODE] as usize
}

#[cfg(test)]
mod answers {
    use std::thread;
    use super::*;
    use test_case::test_case;

    // #[test_case(load_blueprints(SAMPLE_INPUT), 24 => 33; "with example data")]
    // #[test_case(load_blueprints(PERSONAL_INPUT), 24 => 1306; "with real data")]
    pub fn problem1(blueprints: Vec<Blueprint>, minutes: u32) -> usize {
        let mut children = vec![];
        let mut id = 1;

        for blueprint in blueprints {
            let child_id = id;

            children.push(thread::spawn(move || {
                let mut best = 0;
                for _ in 0..2000000 {
                    best = best.max(simulate(&blueprint, minutes));
                }

                best * child_id
            }));

            id += 1;
        }

        children
            .into_iter()
            .map(|child| child.join().unwrap())
            .sum()
    }

    #[test_case(load_blueprints(SAMPLE_INPUT), 32 => 3476; "with example data")]
    //#[test_case(load_blueprints(PERSONAL_INPUT), 24 => 1306; "with real data")]
    pub fn problem2(blueprints: Vec<Blueprint>, minutes: u32) -> usize {
        let mut children = vec![];

        for blueprint in blueprints.into_iter().take(3).collect::<Vec<_>>() {
            children.push(thread::spawn(move || {
                let mut best = 0;
                for _ in 0..10_000_000 {
                    best = best.max(simulate(&blueprint, minutes));
                }
                best
            }));
        }

        children
            .into_iter()
            .map(|child| child.join().unwrap())
            .inspect(|elem| println!("{elem}"))
            .product()
    }

    pub fn load_blueprints(input: &[&str]) -> Vec<Blueprint> {
        input
            .iter()
            .map(|&line| Blueprint::from_str(line))
            .collect()
    }

    const SAMPLE_INPUT: &[&str] = &[
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
    ];

    const PERSONAL_INPUT: &[&str] = &[
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 3 ore and 8 obsidian.",
        "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 11 obsidian.",
        "Blueprint 4: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 2 ore and 11 obsidian.",
        "Blueprint 5: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 9 clay. Each geode robot costs 2 ore and 9 obsidian.",
        "Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 12 clay. Each geode robot costs 2 ore and 10 obsidian.",
        "Blueprint 7: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 3 ore and 14 obsidian.",
        "Blueprint 9: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 8 obsidian.",
        "Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 8 obsidian.",
        "Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 19 obsidian.",
        "Blueprint 12: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.",
        "Blueprint 13: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 6 clay. Each geode robot costs 2 ore and 20 obsidian.",
        "Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 5 clay. Each geode robot costs 3 ore and 18 obsidian.",
        "Blueprint 15: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 4 ore and 7 obsidian.",
        "Blueprint 16: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 4 ore and 11 obsidian.",
        "Blueprint 17: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 16 obsidian.",
        "Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.",
        "Blueprint 19: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 17 obsidian.",
        "Blueprint 20: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 3 ore and 14 obsidian.",
        "Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 6 clay. Each geode robot costs 2 ore and 16 obsidian.",
        "Blueprint 22: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 3 ore and 14 obsidian.",
        "Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 14 obsidian.",
        "Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 4 ore and 13 obsidian.",
        "Blueprint 25: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 12 obsidian.",
        "Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 11 clay. Each geode robot costs 4 ore and 12 obsidian.",
        "Blueprint 27: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 4 ore and 16 obsidian.",
        "Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 19 obsidian.",
        "Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 15 obsidian.",
        ];
}
