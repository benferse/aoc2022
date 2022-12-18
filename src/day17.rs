//! Day 17 - Pyroclastic flow

use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Rock(u32);

const RIGHTMOST_COLUMN: u32 = 0b00000001_00000001_00000001_00000001;
const LEFTMOST_COLUMN: u32 = 0b01000000_01000000_01000000_01000000;
const ROCKS: [Rock; 5] = [
    // horizontal bar, cross, glider, vertical bar, square
    Rock(0b00000000_00000000_00000000_00011110),
    Rock(0b00000000_00001000_00011100_00001000),
    Rock(0b00000000_00000100_00000100_00011100),
    Rock(0b00010000_00010000_00010000_00010000),
    Rock(0b00000000_00000000_00011000_00011000),
];

impl Rock {
    pub fn blow_left(&mut self, horizon: u32) {
        if self.0 & LEFTMOST_COLUMN == 0 {
            let new_position = self.0 << 1;
            if new_position & horizon == 0 {
                self.0 = new_position;
            }
        }
    }

    pub fn blow_right(&mut self, horizon: u32) {
        if self.0 & RIGHTMOST_COLUMN == 0 {
            let new_position = self.0 >> 1;
            if new_position & horizon == 0 {
                self.0 = new_position;
            }
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0.to_be_bytes() {
            write!(f, "|")?;
            for offset in 1..=7 {
                write!(f, "{}", if byte >> (7 - offset) & 0x1 == 1 {'#'} else {'.'})?;
            }
            write!(f, "|\n")?;
        }

        Ok(())
    }
}

pub fn get_horizon(pile: &Vec<u8>, elevation: usize) -> u32 {
    if elevation > pile.len() {
        0
    } else {
        pile[elevation..]
            .iter()
            .take(4)
            .rev()
            .fold(0, |accum, byte| (accum << 8) | *byte as u32)
    }
}

pub fn drop_rock(pile: &mut Vec<u8>, wind: &[u8], windex: &mut usize, rock_index: usize) {
    // `pile` represents the mass of rocks that has come to rest.
    // The new rock will be generated three layers above the top of the pile
    let mut altitude = pile.len() + 3;
    let mut rock = ROCKS[rock_index];

    // Until the rock comes to rest, let the wind blow it around and then
    // try to drop it down a layer. Blocks always start above the top of the pile,
    // and the wind affects them before they drop
    loop {
        let wind_horizon = get_horizon(pile, altitude);

        let next_jet = wind[*windex];
        *windex = (*windex + 1) % wind.len();

        match next_jet {
            c if c == '<' as u8 => rock.blow_left(wind_horizon),
            c if c == '>' as u8 => rock.blow_right(wind_horizon),
            _ => (),
        }

        if altitude > pile.len() {
            // Still above the pile, so just drop
            altitude -= 1;
        } else if altitude == 0 || rock.0 & get_horizon(pile, altitude - 1) != 0 {
            // The rock has come to rest, either because it hit the bottom or the pile
            // is now propping it up. Introduce the bytes of the rock into the pile
            let rock_bytes = rock.0
                .to_le_bytes()
                .into_iter()
                .take_while(|byte| *byte != 0);

            for rock_byte in rock_bytes {
                if altitude < pile.len() {
                    pile[altitude] |= rock_byte;
                } else {
                    pile.push(rock_byte);
                }

                altitude += 1;
            }

            // Next rock please
            break;
        } else {
            // At or below the pile, but we didn't collide with the horizon, so drop
            altitude -= 1;
        }
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT, 2022 => 3068; "with example data")]
    #[test_case(PERSONAL_INPUT, 2022 => 3102; "with real data")]
    pub fn problem1(wind_gusts: &str, num_rocks: usize) -> usize {
        let wind = wind_gusts.as_bytes();
        let mut windex = 0;
        let mut pile: Vec<u8> = vec![]; 
        let mut n = 0;

        while n < num_rocks {
            drop_rock(&mut pile, &wind, &mut windex, n % 5);
            n += 1;
        }

        pile.len()
    }


    const SAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    const PERSONAL_INPUT: &str = "><<<<>>>><<<><<<><<<>><<>>><>><>><<<<>><<<<>><<<>>>><<<><<<>>>><<<<><<><<>><<>>>><<<>>><<<>>>><<>><>><<<<>>><<<>>>><<<<><<>>><<<>>><<>>><>>>><>><>>>><<<>>><<<>><>>><<<<>>><>>>><<<>>>><<<<>><<>>>><<>><>>><<>>><>>>><<<>><<>>><<>>><<<<>>>><>>><>><<<<>>><<<>>><<<<>>>><<<>>><<<>><><<<<>>><<<<><<<>>><<><>>><<<<>>>><>><<<>><<>>><<<<>><<<>>><<>>>><>>>><>>><>>>><<<><<<<>><<>><>>><<>>>><<<<>>>><>>>><>><<<<>>>><<<<>>><<>>><<><>><<<<>><><<<<>>><<<>>><<<>>><<<>><<>>>><<<><<<>>><><<>>>><<<><>>>><<>>><>><<<>>><<<<><<<>><<<>><>>><<<<>>><<>>>><>><<<>>>><<<<>>>><<<>><<<>>>><<>><<>>>><<>>><<<<>>><>>>><>><<<>><<<>>><<<<>>>><<<>>>><>>><>>><>>><><<<>><<<>>><<<<>>>><<<>>>><<>>><<<<>><>><<>>>><>><<>><<<<>>><<<>><<><<<<>><<<<>><<<<><<>><>>><<<><<<><<<<>><<<>>><<>>><>>><<<<>><<><>><<>>><><<<<>>>><<>>>><<<<>>><>><<<<>><<>>>><<<>>>><<<>>>><<><<<<>>>><<<><<<<><<<>>>><<>>><<><>>>><<<>>>><<<<>>>><<<><<<>><<>>>><>>>><>><<<><>>>><<<>>>><<<<>><<<><<<<>>>><<>><>>><<<>>><>><>>>><<<<>>>><<<><<<>><<>>><<<>>><>>><><>>><<>><<>>><<<<>><<><<<>><><<><<>><>>>><<>>>><<<<>>><>>>><>>><<<>>><<<>>>><<><>>><<<>>><>>>><>>>><><<>>>><<<>>><<<>><<>>>><>>>><<><>>>><<<><<<>>>><<<<>>>><<>>><<<<>><<<>><><<<>>>><>>><><><<<><<>><<>>>><<<><<><<<>>>><<<<>>><<><<>>><<<>>>><>>>><<<<>><>><<><<<>><<<>>><<<<>>>><<><<<<>>>><>><<<>>>><<<<><<<<>>>><<>><<>><<><<>>>><<>>><<><<>>>><<>>>><<>>>><<<<><<<<><<<<><<<>>><<<<>>>><>>>><<<>>><<>>><<<>>>><>>><<>>>><<<>><<>>><<><>><<<>><<><<>>>><<>><<<<>><<<>>>><<<>>><<>>><<>>><<>>>><<<<>><>>><>>><><<<><>>><<<>>><><<>>>><<><<<<>><<<<>>><<<<>>><<<<>>><<<>>><<<<>>>><<<<>>><<>>><<<<>><>>>><>>><>><<<><<<><<>><<><<<<><<<<>><<<<><>><<<>>>><<<<>>>><>>><<<>>><>>><<>><<><<>>>><<>><<<<>>><<<>>>><<>>><<>><<<><<<<>>>><<>><>>>><<<<><>>><<>>><>>><<<>>><>><<>><<>>>><<<<><<>><<<<>>>><<<<>>><<<<><>>>><>>><<><>>><<<<>>>><>><<<><><>><><><<<<>><<<<>>><<<><<<<>><>>>><<<><<<<>>><<>>><>>><<<>>><<<<>><><<<>>><<>>>><><<<<><<><<><<<>>><>><>>><<<>><<<<>>>><<<>><<<<><<<<>>><<<<><<>>>><<<>><>><<<>>><<<<>><>>><<<<>><<<<>>><<<<>>>><<>><<<>><<>>><<>>>><>><<><<<>>><<<<><<>><<<<>>><<<>>>><<>><<><>><<<>>><>><<<<>>><<<<><<>><<<>>><<>>>><<<>>>><<<<>><<<<><<<<>>>><<<<>>>><<><<<<>>>><>>>><<<<>>><<<<>><<>><<<<>>><<<<>>>><<>><<>><<<<><<<<>>><<<><<<>><>>>><>>>><<>><<<<>>>><>>>><<<>>>><<<>>><<<<><<<>>>><<>>>><<><<<<>><><<<>>><<<><>><<<<><<<>>>><>><<>>><<<>>>><<>>><<<>><<>>>><<<<>>>><<<<>>><<>>>><<<<>>><<<<>><<>>><<<><>>><<>>><>><<><>>><<<><<>>><<<>>><<<>><<>>><>><>><><<<<>>><<<>><<<<>>>><<<<>><>>><<<<><>>><><<<<>>><<<>>>><>>><<<>><>><<<>>>><<<<><<<>>><<><<>>><<<>>>><><<<>>><<>><<<><<<<>><>>><<><<>>><<<<>><<<<>><<<<><<><>>><><<>>>><<>>>><<<<>>><<<>><<<><<<<>>>><<>>><<<>>><>>><<<><>>>><<<><<<<>>>><>><>><<<>>>><<<>><<>><<>>><<<<>><<<<>>>><<<<>>>><<><>>>><<<<>>>><<<<>><<<<><<<<>>><<<<>><<<>>>><>><<<><<<<>>><<>>>><<<>><<<>>><>>><<<><><<><<>>><>>><<<>><<<>>>><<<<>>>><<<<>><<<>>><<<>><<>><>>>><<<<>><<<>>>><<<>><<<>>>><>>>><>>><<<<><>>><>>><<>><><<<>>>><<<<>>>><<>><<>>><><>><<<<>><<<>><<<>>>><<<>><<<>>><<>>><<<>>>><<><<<<>><>>><<<>>><<><<<<>><<<<>>><<<<>>>><<<><<<<>>><<<<>>>><<<>>><><<>>>><>>>><<<<>>><<<>>><<<<><<<>><<<><<<>><<<>>>><<<<>>>><<>>><<>><<>>>><<<>><><<<>>>><><<<>>>><<><<>>><>>><<<<>><<<<>>>><<<<><<<>>>><<>><<<>><<<>><<<<>><>>>><<<<><<<<>>>><><<>><<>><<<<>><<<<>>>><<<<>>><<><<><<>>>><>>>><<><<<>>><<>>><<>><>><<<><<>>>><<<>>><>>>><>>><<><<><<><>><>>>><>><>><<>><<<>><<>><<<<>>><<<<>>><><<<>>>><><<>><>>><>><<<><<>><<><><<<>>>><<<<>>>><<<<><>>>><<<<>>><<>>>><<<>><<<<>><<>>>><<<<>>>><<>>><<<<>><<<>>><<<<>><<>><<<>>>><>>>><<<<><<<<>>><<<<>><<<<>>>><<<>><<<<>>><>><><<<<><<><<<>><<><<<>>><<<>>><<><<>>><<>>>><<<<>><>>>><<><<>>><<>>>><<><<<<>><>><>><<>><<<>>>><<<<>>>><<<><<<<>>>><<<<>>>><<>><<>>>><<<>>>><>>><<<><<<>>>><<>>><<<<>>>><<<<><<>><<>>><<>><<<>>>><>><>><<<><<<>>><<<><>>>><>>><><<<<>><<<>>>><<<<>><<>><<<>>>><>>>><<>>><<<<>>><<<>>>><<<<>><<<<>>>><<<<>><<<><<>>><<>>><><<<><<<>><<>>>><>><<<>><<>><<<<>>><<><<<>>><<>>>><<<<><<>>>><><>>>><<>>><<<<><<<<>><<<><<<><<><<<>>><<<<><<<>><><<<<>>>><<<<>>>><<<>><<<>>><<<><<><><<>>>><>>><<<<>>>><<<<>>><<>>><<<<><<<<>>><<<><<<<>><<><<><<<>><<>><<>>><<>>><<>>><>>><<<>>>><<<<>>><>>><<<>><<><<>>>><<<<>>>><<<>>>><<<>>><<><<><<<><<<><>>>><><<<<><<<>>>><>>><>>>><>><<>><<<<>>>><<>>><>><>><>>><<<<>>><>><<<<>>>><<<<><<>><>>>><>>><<>>>><<<>>>><<>><<<<><<<>><><<<<>>>><>>>><>>><<<>>>><>>>><<<<>>>><<<>>>><<><<><<>>><<><<>><>>>><<<>>><<<>>><<<>>><><<<<>>><>><<<<><<>>><<>><<<<><<<<>><<>><<>><<>>>><<<>>>><>>><<<>>>><<<>><<<<>>><>><<>>><><<>>><<<>>><>><<>><<<<>><<<>>>><<<<>><<>><<<<>>>><<<>>>><<<<><<<<>><<<<>><<<<>>>><<>><<<<>>>><<<<>>>><>>>><>>><>>><<<<><<<>><<>>>><><<>>><>>>><<<<>>>><>><>>>><>>>><<<><<<>>><<>>>><<<<><>>><<>><<<>>>><<>>>><<>><<<><<<>>><<<><<><<>>><>><>>><<>>>><<<>><<>>><<<>>>><<>><><<>><<<<>>><<<>><<<>>><>><<>>><<<<>>>><<>><<>><<<<>>>><<>>><>>>><>><<>><<>>>><<<>>>><<<>>><<>>>><<><<><<<><<>>>><<>>>><<><>>><<><<<>>>><><<>><<<>><<<>>>><<>><<<<>>>><>><<<>><>>><<<>>><<<>>>><<<<>>><<<>><<<<>>><><<<<>>>><<><<>><<<<><<><<>>>><>><>><>>><<<>>>><<<<><<<>><<>>>><><<<><<>>><<<<><<<<>>>><<<>>>><<<>><<<>><<<><<><<><><<<<>>>><<<<><<<>><<<<><<<<><<<<>>>><<<>><<<>>>><<<<>><>><<>><>>>><>><<>><<><<<><<<>><<<>>><<<<>>><<<>>>><>>><<<><><<<<>>><<<<>><<<<>>>><>>><<<<>>>><<>>>><<<<><<>>>><<<<><<>>><<<>>><<<><<<<>>>><<<<>><<<<>><<<<><<<>>><<<<><<<<>><<<>><<<<>><<>>>><><<>>><>>><<<>><<><<><<>><<><><<>>>><><>>><<<>><>>><<<>>><<<>><<<<>>><<>>><<>>>><<>>><<<<>><<<<><>>>><>>>><<<<>>><<<>><><><<>>>><<><<<>>>><<>><<<><><<<<><<<><<<>><<<<>>><<<<>><<<<>><<<<>>>><<<<>>>><><>>>><<>>><<><<<>>><<<><>>>><<<<>>>><>><<<>><<>><<>>>><<<<><>>><<<<>><<><<>><<<<>>><><<<><<<><<<>>><<>><<<<>>>><<<>>>><<<<><>>>><><>><<<>>><<<<>>>><<<>>>><<>><<><<>><<<<>>><<<<>><<>>>><<>><<<<>>>><<<>>>><<<<>>><<>><<<<><><<>><>>><<><<<>><<<>><<<>>>><<>><<<<>>><<<>>>><>>><<<><<<<>>>><<<<>>><<<>><<<>>><><<<<>>><<>>>><<<>>><>><><>><<<><<<<><<<<>>>><<<>>><><<<><<<>><<>><<><<><>><><<<>><<<<>><<>>>><<<<>>>><><<<<><<<>>>><>>><<<>>>><<<>><>>>><<<>><<<<>>><<<<>><<>><><<<<>><<>>><<<<>><<<>>>><<><>>>><<<<>>>><><>>><<>>>><>><<>>>><><>><<>>><><>>><<<>>>><<<<>>><<<><<<<>>>><<<>><<<<>><<<>><>><<<>>><<<<>><<>>><>><<<<>>>><<<>><<>><>>><<<<>>>><><<><<<>>><<<<>><<<<>><><>>><<>><>>>><<<<>>>><<<>>><<<>><<<<>>>><<<>><<>>><<<>>><>>><<<><<<<>>>><<><<<<><<<>>>><>>>><<<>><>>>><><<<<>><<<<>>>><<<>>><>>>><>><<<>>>><<><<<>>><<><<<>>>><<>><<>>><<<>>>><<<<>><>>><<<<><<<<>>>><>><<<>>><<><>><<<>><>>><<<>>><<<<>>>><<<>>>><>>><<><<<<>>>><><<<>>>><<>>><><<<>>><<<<>><<<>>><<>>><<<><<><<<>><<>><<<<>>><<<><><>><<>>><<<<>>><>>>><<><<<<>>>><<<<><<<>>><<<>><<>>>><>>>><<<><<>>><<<>>>><<><<<>>><><<<<>><<<<><<<<>><<>><<<<>>>><<<>><<<<><<<<>>><<>>><<<>>><<<<><<<<>>><<<<>>>><<<>>><<><<<<>>><<<<>>><><<<>>><<><<<>><>>><<<<>>>><<<<>>><<>><<<<>><<<><<<<><<>>>><<<<>>>><<>>>><<<<>>><<<>>>><<>>><<>>><<<<><<<<>>>><><<<>>><<<>>><><<<<>>><<>>>><<<<>>><<<>><<>>>><<<<>><<>><<<<>><<<<>>>><<<><>>>><<<><<<<>><>>><>><<<<>>><>>>><>><>><<<<>>>><<<<><>>><<<<>><>><><><<>><<<>><<<><>>><<<<><<<>><<<<>><<<<>>><<>>>><<><<<<>><<<>>><<<>><>><>>>><<<<>>><<>><<>><<>>>><<<<>>>><<<<><<>><>><>>>><<<>>><<<<>><<>>>><<>>><>>>><<>>>><<<<>>>><<<<>>><<>><<<>>><<>>>><<>>>><<<><<<>>>><<<<>>>><<<><<><<>>>><>>><<>><<>><<<>>>><<<><<>>><<><<<>>>><<<>>><>>><<<>><<<>><>>><<<<>><<<>>>><<><<>>><>>><>><<>><<<>>>><<>><<<<><><<<><<>>>><<>><<<>>>><<<><<<<><<><<<<>>>><<<<>><<><>>><<<><><<<<>><<>><<<<><>><<><<<>><<<>>>><>><>>><<<><><<<>><<<>><>>><>><<<<>><<<>>><<<>>>><>><>>>><<>><<<>>><<>>><<><<>><<>><><<<>>><<<>><<<<>><<<>>><<<<>><<<>>><<<>>><<<<><<<<>>><>><<<<>>><<<>>>><<<>><<>>><<>>><>>><<><<<><<<<><<>>>><<<><<>>><>>>><<<>>>><>>>><<><<<>>><>>><<>>><<>>>><>>>><>>><<>><<<>>><<<>><>><>>>><<<><>>><>><>>><>>><<<>>>><<>>>><<<>>><<<<>><>><><<>>>><>><<><<<<><><><<>>>><<<<>>><>>><<<>>>><<<>>><>>><<<>><<<>>><<<<>><<>><<<<>>><<<>><<<<>>><<<<>><>><>>>><>>><>><<<<>><<<<><>>><>><>>><<<<>>>><<<><<>>><<>>>><<<<>><<<<><>><>><<>>><<>><<><>><><<>>>><<<<>>>><<<<>>>><<>>><<<<><>>>><<<<>>><<<>><<>>>><>>><<<>>><<>>>><<<>><<<><<<<>>>><<<>>>><<<<>><<<><<<<>>>><><><><>>><<<>><<<>><<<>>><<>><<<<>><<<<>>><<<>><<<<>><>><<<>><<<>>>><<<>>>><<>><<<<>>><<>>>><<>><<>>>><<<<>>>><<<>>><<>>>><<<>>>><<><<<<><<>><<<>><<<>><<><<<>><><>>><<>>><<><>>><<<<>>>><<<>>><<>>>><<<>>>><<<>>>><<<<>><<<<>><<>><>>><<>><<><<<>>><<<>><<><<>><<><<<<>>><>>><<><<<>>>><<>>>><>>><<>>><<><><<<<>>><<><<<>><<<><<>>><<<<>>>><<>>>><><>>><<<>>>><<<>><<<<><<<<><<<<>>><><<<>><<<<>>><<<<><<<>>><<<>><<<<>><<<<>>>><<<<><>><<<>>><<<<><<><<<<><<<<>>><<>>><>>>><<<>>><>>><<>><>><<<<><<<<>>>><<<>>><<<><<>>>><<><<<<>><<>>>><<<>><<<<><<<<>>><>><><<<>>>><<<<>><>>>><>>><<<<>>>><<>><<>><>>><<><<><>>>><<<<>>><<<>><>>><<<><>>><<<<>><<<<>>>><>>><>>><>>>><>><<>>><<<<>><>>><<><<<>>><<>>>><>>>><<>>><<<<>>><<><<<><<<><<<<>>>><<<>>>><<<>>><<<<>>>><<<<>>>><<>><<<<>>><<>>><><<>><<><>>><<>>><>>>><<<>>>><<<<>>><<<><<<<><>>><<><>>>><<<<>>>><<<>>><>>>><<<<>>><<<>>>><<>>><<<><<>>>><<>>>><<<<><>>>><<>>><<<<><<<><>>>><<<<>>><<<>><>><<>><>>><>>><<<>><<><>><<<<>><<<<><<<<>>><<<>>>><<<<>>>><>>>><<<<>><<<<><<<<>>>><<<<>>><<<>>>><>>><<>>>><<<>><>>>><<<<>>>><<<>><<>>>><<<>>>><<<><<>>>><<<<>>>><<<<>><<<<>>>><>>><<>>><<<>>>><<<>>><<<>><>><<>><>>><<>><<<>><<<><<<<>><<<<>>><<<<>>>><<<<>>><<<>><<<<><<<><<<><<<>>><<>><>><>>><<>>><<<<><<<<>><>>><>>><<<><<<<>>><>>>><<>><<<<>>>><<>>><<<><><>>>><<>>>><><>>>><<><<<<>><<<<>><<<>><>>><<>>><<<<>>>><<<<>><<<<><<><>>>><<<>><<>><<<>>>><><<<><<<>>><>>>><<>>><<<<>>>><><>><>>><<<<>><<><>>>><<<>><<<<><<<>>><>>><<><<<>>><<<<>><<<>><<<>>><>>>><<<>>>><<<>>>><<<>>><<<<>><<<<>><<<<>><<<<>>>><<<>><>><>><<>>>><>>>><><<<<>><<><<>><>><>>>><<>>>><<>>><<>>>><<>><<<>><<<<>>>><<<<><>>><<<<><<><<<><<<>><><<<>>>><<>>><<<>>>><>>><<<<>>><<<>>><<<>>><>>><<<<>><<>>>><><>>><<>>>><<>>><<>>><><<<>>><<<<><<<>>>><<<><<<<>><>>><<<>>><>>><<><<><<>>>><<>>><<<><<><<>>>><<<>><<>><<<<>><>>>><>><>><><>><><<>>>><>><<<>>>><<><<<>><<<<>><<><<<<><<><>>><<>>><<>><<<>>>><<<<>>>><<<<>>>><<<<>><<<<><<<>>><<<><<>><><<>>>><<><>><>>>><>>>><<>>><<><<<>>>><<<<>>>><>>>><>><<<<>>>><>>>><<<>><<>><<<<><<>>>><<<>>>><>>>><<>>>><>><><<<<>>>><<<><<>><<<<>>>><<<>>>><<<<><<<>><>>>><<<<>><<<><<<<>>><<<>><<>>>><<<<>><<<><<>><<<>";
}
