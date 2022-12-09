//! Day 8 - Treetop Tree House

use grid::Grid;

#[derive(Clone, Debug)]
pub struct Node {
    height: u32,
    visibility: Option<bool>,
}

impl Node {
    pub fn new(height: char) -> Self {
        Self {
            height: height.to_digit(10).expect("0-9 only pls"),
            visibility: None,
        }
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.height.eq(&other.height)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

pub fn load_grid(lines: Vec<&str>) -> Grid<Node> {
    let width = lines[0].len();

    let content = lines
        .iter()
        .flat_map(|line| line.chars().map(Node::new))
        .collect();

    Grid::from_vec(content, width)
}

pub fn with_visibility(grid: &Grid<Node>, vis: Option<bool>) -> usize {
    grid
        .iter()
        .filter(|node| node.visibility == vis)
        .count()
}

fn assess_tree(tallest: Option<u32>, tree: &mut Node) -> Option<u32> {
    match tallest {
        // If there's no previous tallest tree, this one must be tallest
        None => {
            tree.visibility = Some(true);
            Some(tree.height)
        },
        Some(tallest) => {
            // If we already know the tree is visible, we don't care if whether
            // it's visible or not from some other direction. So only bother if
            // we haven't checked this tree, or if we haven't found it to be
            // visible from some other direction yet
            if tree.visibility != Some(true) {
                tree.visibility = Some(tree.height > tallest);
            }

            Some(tallest.max(tree.height))
        }
    }
}

/// Given a heightmap of the trees, figure out which are visible and which are
/// hidden
///
/// # Examples
///
/// ```
/// use aoc2022::day8::*;
/// 
/// let mut sample = load_grid(vec![
///     "30373",
///     "25512",
///     "65332",
///     "33549",
///     "35390",
/// ]);
///
/// categorize_trees(&mut sample);
/// assert_eq!(with_visibility(&sample, Some(true)), 21);
/// ```
pub fn categorize_trees(heightmap: &mut Grid<Node>) {
    for idx in 0..heightmap.rows() {
        heightmap.iter_row_mut(idx).fold(None, assess_tree);
        heightmap.iter_row_mut(idx).rfold(None, assess_tree);
    }

    for idx in 0..heightmap.cols() {
        heightmap.iter_col_mut(idx).fold(None, assess_tree);
        heightmap.iter_col_mut(idx).rfold(None, assess_tree);
    }
}

/// Given a heightmap of the trees, figure out the score of the best view
///
/// # Examples
///
/// ```
/// use aoc2022::day8::*;
/// 
/// let mut sample = load_grid(vec![
///     "30373",
///     "25512",
///     "65332",
///     "33549",
///     "35390",
/// ]);
///
/// assert_eq!(best_scenic_score(&sample), 8);
/// ```
pub fn best_scenic_score(heightmap: &Grid<Node>) -> usize {
    let (rows, cols) = heightmap.size();
    let mut best_score = 1;

    // Visit every node, cast rays in each cardinal direction, and count
    // how many trees we can see, noting that since anything on the periphery
    // will have at least one score of zero, there's no point in calculating them
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if let Some(this_node) = heightmap.get(row, col) {
                let this_row = heightmap.iter_row(row).collect::<Vec<_>>();
                let this_col = heightmap.iter_col(col).collect::<Vec<_>>();

                let (left, right) = this_row.split_at(col);
                let (up, down) = this_col.split_at(row);

                // Mixing indexes and offsets is always a blast -_-
                let right_score = right.iter().skip(1).position(|&x| x >= this_node).map_or(right.len() - 1, |p| p + 1);
                let left_score = left.iter().rposition(|&x| x >= this_node).map_or(left.len(), |p| left.len() - p);
                let down_score = down.iter().skip(1).position(|&x| x >= this_node).map_or(down.len() - 1, |p| p + 1);
                let up_score = up.iter().rposition(|&x| x >= this_node).map_or(up.len(), |p| up.len() - p);

                // No, I don't care about overflow
                best_score = best_score.max(right_score * left_score * down_score * up_score);
            }
        }
    }

    best_score
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    fn problem_1_and_2() {
        let input = include_str!("./input/day8.txt").lines().collect();
        let mut forest = load_grid(input);
        
        categorize_trees(&mut forest);

        let num_visible = forest.iter().filter(|node| node.visibility == Some(true)).count();
        assert_eq!(num_visible, 1779);

        let best_view = best_scenic_score(&forest);
        assert_eq!(best_view, 172224);
    }
}
