//! Day 12 - Hill climbing algorithm

use std::collections::{HashSet, VecDeque};

pub type HeightMap = Vec<Vec<u8>>;
pub type Coordinate = (isize, isize);

pub fn load_grid(lines: &[&str]) -> (HeightMap, Coordinate, Coordinate) {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (u, row) in lines.iter().enumerate() {
        let grid_col = row
            .chars()
            .enumerate()
            .inspect(|(v, c)| {
                if *c == 'S' {
                    start = (*v as isize, u as isize);
                } else if *c == 'E' {
                    end = (*v as isize, u as isize);
                }
            })
            .map(|(_, c)| {
                match c {
                    'S' => b'a',
                    'E' => b'z',
                    c => c as u8,
                }
            })
            .collect();

        grid.push(grid_col);
    }

    (grid, start, end)
}

pub fn do_bfs(grid: &Vec<Vec<u8>>, start: (isize, isize), end: (isize, isize)) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, start));
    visited.insert(start);

    while let Some((distance, current)) = queue.pop_front() {
        let (cur_x, cur_y) = current;
        for (x, y) in [(cur_x - 1, cur_y), (cur_x + 1, cur_y), (cur_x, cur_y - 1), (cur_x, cur_y + 1)] {
            let ux = x as usize;
            let uy = y as usize;

            if x >= 0 && y >= 0 && uy < grid.len() && ux < grid[uy].len() && grid[uy][ux] <= grid[cur_y as usize][cur_x as usize] + 1 && visited.insert((x, y)) {
                if (x, y) == end {
                    return distance + 1;
                }

                queue.push_back((distance + 1, (x, y)));              
            }
        }
    }

    0
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_GRID => 31; "with example data")]
    #[test_case(personal_grid().as_slice() => 534; "with personal data")]
    fn problem1(input: &[&str]) -> u32 {
        let (grid, start, end) = load_grid(input);
        do_bfs(&grid, start, end)
    }

    #[test_case(SAMPLE_GRID => 29; "with example data")]
    #[test_case(personal_grid().as_slice() => 525; "with personal data")]
    fn problem2(input: &[&str]) -> u32 {
        let (grid, _, end) = load_grid(input);
        let mut shortest = u32::MAX;
        let mut candidates = vec![];

        for (i, line) in grid.iter().enumerate() {
            for (j, k) in line.iter().enumerate() {
                if *k == b'a' {
                    candidates.push((j as isize, i as isize));
                }
            }
        }

        for candidate in candidates {
            let q = do_bfs(&grid, candidate, end);
            if q > 0 {
                shortest = shortest.min(q);
            }
        }

        shortest
    }

    const SAMPLE_GRID: &[&str] = &[
        "Sabqponm",
        "abcryxxl",
        "accszExk",
        "acctuvwj",
        "abdefghi",
    ];

    fn personal_grid() -> Vec<&'static str> {
        include_str!("./input/day12.txt")
            .lines()
            .map(str::trim)
            .collect()
    }
}
