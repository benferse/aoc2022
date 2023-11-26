//! Boiling boulders

use std::collections::BTreeSet;

pub type Point = (i32, i32, i32);
pub type Extents = (i32, i32);
pub type Droplet = BTreeSet<Point>;

pub trait PointExt {
    fn x_left(&self) -> Self;
    fn x_right(&self) -> Self;

    fn y_left(&self) -> Self;
    fn y_right(&self) -> Self;

    fn z_left(&self) -> Self;
    fn z_right(&self) -> Self;
}

impl PointExt for (i32, i32, i32) {
    fn x_left(&self) -> Self {
        (self.0 - 1, self.1, self.2)
    }

    fn x_right(&self) -> Self {
        (self.0 + 1, self.1, self.2)
    }

    fn y_left(&self) -> Self {
        (self.0, self.1 - 1, self.2)
    }

    fn y_right(&self) -> Self {
        (self.0, self.1 + 1, self.2)
    }

    fn z_left(&self) -> Self {
        (self.0, self.1, self.2 - 1)
    }

    fn z_right(&self) -> Self {
        (self.0, self.1, self.2 + 1)
    }
}

pub fn calculate_extents(droplet: &Droplet) -> (Extents, Extents, Extents) {
    let mut x = (i32::MAX, i32::MIN);
    let mut y = (i32::MAX, i32::MIN);
    let mut z = (i32::MAX, i32::MIN);

    for atom in droplet {
        x.0 = x.0.min(atom.0 - 1);
        x.1 = x.1.max(atom.0 + 1);

        y.0 = y.0.min(atom.1 - 1);
        y.1 = y.1.max(atom.1 + 1);

        z.0 = z.0.min(atom.2 - 1);
        z.1 = z.1.max(atom.2 + 1);
    }

    (x, y, z)
}

pub fn total_free_faces(droplet: &Droplet) -> usize {
    let mut free_faces = droplet.len() * 6;
    for atom in droplet {
        for offset in [1, -1] {
            if droplet.contains(&(atom.0 + offset, atom.1, atom.2)) {
                free_faces -= 1;
            }

            if droplet.contains(&(atom.0, atom.1 + offset, atom.2)) {
                free_faces -= 1;
            }

            if droplet.contains(&(atom.0, atom.1, atom.2 + offset)) {
                free_faces -= 1;
            }
        }
    }

    free_faces
}

pub fn total_trapped_faces(droplet: &Droplet) -> usize {
    // Calculate the extents of the droplet in the coordinate space
    // to come up with a bounding volume
    let ((x_min, x_max), (y_min, y_max), (z_min, z_max)) = calculate_extents(droplet);

    // Find all of the coordinates reachable in the bounding volume
    // without passing through the droplet
    let mut volume = BTreeSet::new();
    let mut queue = BTreeSet::new();
    queue.insert((x_min, y_min, z_min));

    while let Some(current) = queue.pop_first() {
        volume.insert(current);

        let x_left = current.x_left();
        if current.0 > x_min && !volume.contains(&x_left) && !droplet.contains(&x_left) {
            queue.insert(x_left);
        }

        let x_right = current.x_right();
        if current.0 < x_max && !volume.contains(&x_right) && !droplet.contains(&x_right) {
            queue.insert(x_right);
        }

        let y_left = current.y_left();
        if current.1 > y_min && !volume.contains(&y_left) && !droplet.contains(&y_left) {
            queue.insert(y_left);
        }

        let y_right = current.y_right();
        if current.1 < y_max && !volume.contains(&y_right) && !droplet.contains(&y_right) {
            queue.insert(y_right);
        }

        let z_left = current.z_left();
        if current.2 > z_min && !volume.contains(&z_left) && !droplet.contains(&z_left) {
            queue.insert(z_left);
        }

        let z_right = current.z_right();
        if current.2 < z_max && !volume.contains(&z_right) && !droplet.contains(&z_right) {
            queue.insert(z_right);
        }
    }

    let mut trapped_faces = 0;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                let pocket = (x, y, z);
                if !(volume.contains(&pocket) || droplet.contains(&pocket)) {
                    for offset in [1, -1] {
                        if droplet.contains(&(pocket.0 + offset, pocket.1, pocket.2)) {
                            trapped_faces += 1;
                        }

                        if droplet.contains(&(pocket.0, pocket.1 + offset, pocket.2)) {
                            trapped_faces += 1;
                        }

                        if droplet.contains(&(pocket.0, pocket.1, pocket.2 + offset)) {
                            trapped_faces += 1;
                        }
                    }
                }
            }
        }
    }

    trapped_faces
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(sample_input() => 64; "with example data")]
    #[test_case(personal_input() => 4370; "with real data")]
    pub fn problem1(input: Droplet) -> usize {
        let droplet = BTreeSet::from_iter(input);
        total_free_faces(&droplet)
    }

    #[test_case(sample_input() => 58; "with example data")]
    #[test_case(personal_input() => 2458; "with real data")]
    pub fn problem2(input: Droplet) -> usize {
        let droplet = BTreeSet::from_iter(input);
        total_free_faces(&droplet) - total_trapped_faces(&droplet)
    }

    const SAMPLE_INPUT: &[Point] = &[
        (2,2,2),
        (1,2,2),
        (3,2,2),
        (2,1,2),
        (2,3,2),
        (2,2,1),
        (2,2,3),
        (2,2,4),
        (2,2,6),
        (1,2,5),
        (3,2,5),
        (2,1,5),
        (2,3,5),
    ];

    fn sample_input() -> BTreeSet<Point> {
        SAMPLE_INPUT
            .iter()
            .copied()
            .collect()
    }

    fn personal_input() -> BTreeSet<Point> {
        include_str!("./input/day18.txt")
            .lines()
            .map(|line| {
                let tokens = line.split(',').collect::<Vec<_>>();
                (tokens[0].parse().unwrap(), tokens[1].parse().unwrap(), tokens[2].parse().unwrap())
            })
            .collect()
    }
}
