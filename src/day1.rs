//! Day1 - Calorie counting

/// Given a complete inventory list of all of the elves'
/// snacks, determine how many calories in total are being carried
/// by the elf with the most delicious treats
///
/// Examples
/// ```
/// use aoc2022::day1::sort_inventories;
/// let input = [1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000];
/// let sorted = sort_inventories(&input);
/// assert_eq!(sorted, [24000, 11000, 10000, 6000, 4000]);
/// // No items in inventory is morally equivalent to zero calories
/// assert_eq!(sort_inventories(&[]), [0]); 
/// ```
pub fn sort_inventories(inventory: &[u32]) -> Vec<u32> {
    // Split the full inventory list into each elf's
    // individual inventory and sum them up
    let mut inventory_totals: Vec<u32> = inventory
        .split(|&item| item == 0)
        .map(|snacks| snacks.iter().sum())
        .collect();

    inventory_totals.sort_unstable_by(|a, b| b.cmp(a));
    inventory_totals
}

#[cfg(test)]
mod answers {
    use super::*;
    use std::sync::LazyLock;

    static INPUT: LazyLock<Vec<u32>> = LazyLock::new(|| {
        include_str!("./input/day1.txt")
            .lines()
            .map(|line| line.parse().unwrap_or(0))
            .collect()
    });

    #[test]
    fn problem_1_and_2() {
        let sorted = sort_inventories(&INPUT);

        assert_eq!(sorted[0], 65912);
        assert_eq!(sorted[0] + sorted[1] + sorted[2], 195625);
    }
}
