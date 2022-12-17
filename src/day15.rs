//! Beacon exclusion zone

use std::collections::HashMap;

pub type Point = (i32, i32);
pub type Range = (i32, i32);
pub type SensorMap = HashMap<Point, Point>;

pub fn parse_input(input: &[&str]) -> SensorMap {
    let points = input
        .iter()
        .map(|&s| line_to_points(s));

    SensorMap::from_iter(points)
}

pub fn line_to_points(line: &str) -> (Point, Point) {
    let tokens = line
        .split(&['=', ':', ','])
        .collect::<Vec<_>>();

    let sensor = (tokens[1].parse().unwrap(), tokens[3].parse().unwrap());
    let beacon = (tokens[5].parse().unwrap(), tokens[7].parse().unwrap());

    (sensor, beacon)
}

pub fn count_covered_cells(map: &SensorMap, target_row: i32) -> Vec<Range> {
    let mut ranges = vec![];

    for (sensor, beacon) in map {
        let beacon_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let to_target_row = (sensor.1 - target_row).abs();
        if to_target_row < beacon_distance {
            let width = beacon_distance - to_target_row;
            ranges.push((sensor.0 - width, sensor.0 + width));
        }
    }
 
    ranges
}

/// # Examples
///
/// ```
/// use aoc2022::day15::*;
///
/// assert_eq!(reduce_ranges(vec![(1,3), (3,4)]), vec![(1,4)]);
/// assert_eq!(reduce_ranges(vec![(1,5), (2, 3)]), vec![(1,5)]);
/// assert_eq!(reduce_ranges(vec![(1,2), (3, 4)]), vec![(1,4)]);
/// assert_eq!(reduce_ranges(vec![(1,3), (1,2)]), vec![(1,3)]);
/// ```
pub fn reduce_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort();
    let mut reduced: Vec<Range> = vec![];

    'outer: for i in ranges {
        for j in &mut reduced {
            let overlaps_start = i.0 >= j.0 && i.0 <= j.1 + 1;
            let overlaps_end = i.1 >= j.0 - 1 && i.1 <= j.1;

            match (overlaps_start, overlaps_end) {
                (true, true) => (),
                (true, false) => j.1 = i.1,
                (false, true) => j.0 = i.0,
                (false, false) => continue,
            }

            continue 'outer;
        }
        reduced.push(i);
    }
    reduced
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT, 10 => 26; "with example data")]
    #[test_case(personal_input().as_slice(), 2000000 => 5335787; "with real data")]
    fn problem1(input: &[&str], target_row: i32) -> i32 {
        let sensor_map = parse_input(input);
        let ranges = count_covered_cells(&sensor_map, target_row);
        reduce_ranges(ranges)
            .iter()
            .map(|range| range.1 - range.0)
            .sum()
    }

    #[test_case(SAMPLE_INPUT, 20 => 56000011; "with example data")]
    #[test_case(personal_input().as_slice(), 4000000 => 13673971349056; "with real data")]
    fn problem2(input: &[&str], max_y: i32) -> i64 {
        let sensor_map = parse_input(input);
        for y in 0..max_y {
            let ranges = count_covered_cells(&sensor_map, y);
            let ranges = reduce_ranges(ranges);
            if ranges.len() == 2 {
                let x = ranges[0].1 + 1;
                return x as i64 * 4000000 + y as i64;
            }
        }

        0
    }

    const SAMPLE_INPUT: &[&str] = &[
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day15.txt")
            .lines()
            .collect()
    }
}
