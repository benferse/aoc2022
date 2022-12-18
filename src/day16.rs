//! Day 16 - Proboscidea Volcanium

use std::collections::HashMap;

pub type AdjacencyMatrix = HashMap::<String, Vec<String>>;
pub type NonzeroValves = HashMap::<String, i32>;
pub type ValveSelector = HashMap::<String, u32>;
pub type ShortestPaths = HashMap::<String, HashMap<String, i32>>;
pub type FlowPerPath = HashMap::<u32, u32>;

#[derive(Default)]
pub struct Caves {
    pub useful_valves: HashMap::<String, i32>,
    pub selector: HashMap::<String, u32>,
    pub paths: HashMap::<String, HashMap<String, i32>>,
}

impl Caves {
    pub fn create(input: &[&str]) -> Self {
        let mut caves = Self::default();

        let mut adjacency_matrix = AdjacencyMatrix::new();

        for line in input {
            let tokens = line
                .split_whitespace()
                .collect::<Vec<_>>();
            
            let name = String::from(tokens[1]);
            let tunnels = tokens[9..]
                .iter()
                .map(|tunnel| String::from(&tunnel[0..2]))
                .collect::<Vec<_>>();
            adjacency_matrix.insert(name.clone(), tunnels);

            let flow_rate: i32 = tokens[4]
                .split(&['=', ';'])
                .collect::<Vec<_>>()[1]
                .parse()
                .unwrap();

            if flow_rate != 0 {
                caves.useful_valves.insert(name.clone(), flow_rate);
            }
        }

        // And the masks for each interesting node
        let masks = caves.useful_valves
            .iter()
            .enumerate()
            .map(|(idx, (valve, _))| {
                (valve.clone(), 1u32 << idx)
            });

        caves.selector = ValveSelector::from_iter(masks);

        // Initialize and fill the shortest paths table
        for j in &adjacency_matrix {
            let mut paths = HashMap::new();

            for k in &adjacency_matrix {
                let initial_distance = if j.1.contains(k.0) { 1 } else { i32::MAX };
                paths.insert(k.0.clone(), initial_distance);
            }

            caves.paths.insert(j.0.clone(), paths);
        }

        let all_valves = adjacency_matrix.keys().collect::<Vec<_>>();
        for &u in &all_valves {
            for &v in &all_valves {
                for &w in &all_valves {
                    let x = caves.paths[v][w];
                    let z = caves.paths[v][u].saturating_add(caves.paths[u][w]);
                    let y = caves.paths.get_mut(v).unwrap();
                    y.insert(w.clone(), x.min(z));
                }
            }
        }

        caves
    }

    pub fn solve(&mut self, start_from: &str, open_valve_mask: u32, time_left: i32, accumulated_flow: i32, solution: &mut HashMap<u32, i32>) {
        // Update the best flow for the valves used so far
        let current_best = solution.get(&open_valve_mask).unwrap_or(&0);
        solution.insert(open_valve_mask, *current_best.max(&accumulated_flow));

        // For every remaining useful valve, recursively visit them depth-first until we
        // run out of time
        let useful_valves = self.useful_valves.clone();
        for (node, flow) in &useful_valves {
            let new_deadline = time_left - self.paths[start_from][node] - 1;
            let node_mask = self.selector.get(node).unwrap();
            if node_mask & open_valve_mask != 0 || new_deadline <= 0 {
                continue;
            }

            self.solve(node, open_valve_mask | node_mask, new_deadline, accumulated_flow + new_deadline * flow, solution);
        }
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT => 1651; "with example data")]
    #[test_case(PERSONAL_INPUT => 1880; "with real data")]
    fn problem1(input: &[&str]) -> i32 {
        let mut caves = Caves::create(input);
        let mut paths = HashMap::new();

        caves.solve("AA", 0, 30, 0, &mut paths);

        let mut solutions = paths.values().collect::<Vec<_>>();
        solutions.sort_by(|a, b| b.cmp(a));

        *solutions[0]
    }

    #[test_case(SAMPLE_INPUT => 1707; "with example data")]
    #[test_case(PERSONAL_INPUT => 2520; "with real data")]
    fn problem2(input: &[&str]) -> i32 {
        let mut caves = Caves::create(input);
        let mut paths = HashMap::new();

        caves.solve("AA", 0, 26, 0, &mut paths);

        let individual_solutions = paths.iter().collect::<Vec<_>>();
        let mut paired_solutions = vec![];

        for (&a, &b) in &individual_solutions {
            for (&c, &d) in &individual_solutions {
                if a & c == 0 {
                    paired_solutions.push(b + d);
                }
            }
        }
        paired_solutions.sort_by(|a, b| b.cmp(a));
        paired_solutions[0]
    }

    const SAMPLE_INPUT: &[&str] = &[
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II",
    ];

    const PERSONAL_INPUT: &[&str] = &[
        "Valve VR has flow rate=11; tunnels lead to valves LH, KV, BP",
        "Valve UV has flow rate=0; tunnels lead to valves GH, RO",
        "Valve OH has flow rate=0; tunnels lead to valves AJ, NY",
        "Valve GD has flow rate=0; tunnels lead to valves TX, PW",
        "Valve NS has flow rate=0; tunnels lead to valves AJ, AA",
        "Valve KZ has flow rate=18; tunnels lead to valves KO, VK, PJ",
        "Valve AH has flow rate=0; tunnels lead to valves ZP, DI",
        "Valve SA has flow rate=0; tunnels lead to valves VG, JF",
        "Valve VK has flow rate=0; tunnels lead to valves RO, KZ",
        "Valve GB has flow rate=0; tunnels lead to valves XH, AA",
        "Valve AJ has flow rate=6; tunnels lead to valves IC, OH, ZR, NS, EM",
        "Valve PJ has flow rate=0; tunnels lead to valves KZ, SP",
        "Valve KO has flow rate=0; tunnels lead to valves KZ, LE",
        "Valve AA has flow rate=0; tunnels lead to valves TW, GB, TI, NS, UL",
        "Valve TW has flow rate=0; tunnels lead to valves TU, AA",
        "Valve VG has flow rate=25; tunnel leads to valve SA",
        "Valve BP has flow rate=0; tunnels lead to valves RO, VR",
        "Valve XH has flow rate=0; tunnels lead to valves GB, RI",
        "Valve TX has flow rate=0; tunnels lead to valves RI, GD",
        "Valve IR has flow rate=10; tunnels lead to valves TN, NY, JF",
        "Valve TU has flow rate=0; tunnels lead to valves JD, TW",
        "Valve KC has flow rate=0; tunnels lead to valves SP, RO",
        "Valve LN has flow rate=0; tunnels lead to valves EM, RI",
        "Valve HD has flow rate=0; tunnels lead to valves FE, SC",
        "Valve KE has flow rate=0; tunnels lead to valves OM, RI",
        "Valve VY has flow rate=0; tunnels lead to valves PW, BS",
        "Valve LH has flow rate=0; tunnels lead to valves OM, VR",
        "Valve EM has flow rate=0; tunnels lead to valves AJ, LN",
        "Valve SO has flow rate=22; tunnels lead to valves ZP, FE",
        "Valve EC has flow rate=0; tunnels lead to valves OM, UL",
        "Valve KV has flow rate=0; tunnels lead to valves SP, VR",
        "Valve FE has flow rate=0; tunnels lead to valves SO, HD",
        "Valve TI has flow rate=0; tunnels lead to valves AA, PW",
        "Valve SC has flow rate=14; tunnel leads to valve HD",
        "Valve ZP has flow rate=0; tunnels lead to valves SO, AH",
        "Valve RO has flow rate=19; tunnels lead to valves UV, BP, VK, KC",
        "Valve ZR has flow rate=0; tunnels lead to valves OM, AJ",
        "Valve JL has flow rate=21; tunnels lead to valves GN, TN",
        "Valve PW has flow rate=9; tunnels lead to valves TI, GN, VY, GD, IC",
        "Valve UL has flow rate=0; tunnels lead to valves EC, AA",
        "Valve GN has flow rate=0; tunnels lead to valves JL, PW",
        "Valve TN has flow rate=0; tunnels lead to valves JL, IR",
        "Valve NV has flow rate=0; tunnels lead to valves RI, JD",
        "Valve DI has flow rate=23; tunnels lead to valves LE, AH",
        "Valve IC has flow rate=0; tunnels lead to valves PW, AJ",
        "Valve JF has flow rate=0; tunnels lead to valves SA, IR",
        "Valve LE has flow rate=0; tunnels lead to valves DI, KO",
        "Valve BS has flow rate=0; tunnels lead to valves JD, VY",
        "Valve JD has flow rate=15; tunnels lead to valves NV, TU, BS",
        "Valve SP has flow rate=24; tunnels lead to valves KC, KV, PJ",
        "Valve NY has flow rate=0; tunnels lead to valves IR, OH",
        "Valve OM has flow rate=7; tunnels lead to valves EC, GH, KE, ZR, LH",
        "Valve GH has flow rate=0; tunnels lead to valves OM, UV",
        "Valve RI has flow rate=3; tunnels lead to valves NV, KE, LN, XH, TX",
    ];
}
