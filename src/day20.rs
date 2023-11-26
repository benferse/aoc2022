//! Day 20 - Grove Positioning System

/// # Examples
///
/// ```
/// use aoc2022::day20::*;
/// let sample = [1, 2, -3, 3, -2, 0, 4];
/// let result = decrypt(&sample, 1, 1);
///
/// assert_eq!(result, vec![1, 2, -3, 4, 0, 3, -2]);
/// ```
pub fn decrypt(encrypted: &[isize], key: isize, rounds: usize) -> Vec<isize> {
    let encrypted: Vec<_> = encrypted
        .iter()
        .map(|x| x * key)
        .collect();
    let len = encrypted.len();

    // Create an array of indicies first
    let mut indices: Vec<_> = (0..len).collect();

    // Use each item in the encrypted text to mix the contents of
    // the index array
    for _ in 0..rounds {
        for (i, item) in encrypted.iter().enumerate() {
            for j in 0..len {
                if indices[j] == i {
                    indices.remove(j);
                    
                    // indices is temporarily one element shorter than it used to be
                    // so be sure to account for that all over the place :)
                    let k = (j as isize + item - 1).rem_euclid(len as isize - 1) + 1;
                    indices.insert(k as usize, i);
                    break;
                }
            }
        }
    }

    // Now that the array of indices has been mixed, use it as
    // a key to determine how to copy items from the cipher text
    // into the decrypted result
    indices
        .into_iter()
        .map(|b| encrypted[b])
        .collect()
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT, 1, 1 => 3; "1 with example data")]
    #[test_case(SAMPLE_INPUT, 811589153, 10 => 1623178306; "2 with example data")]
    #[test_case(personal_input().as_slice(), 1, 1 => 5962; "1 with real data")]
    #[test_case(personal_input().as_slice(), 811589153, 10 => 9862431387256; "2 with real data")]
    fn problem(input: &[isize], key: isize, rounds: usize) -> isize {
        let decrypted = decrypt(input, key, rounds);

        let zero_idx = decrypted
            .iter()
            .enumerate()
            .find_map(|(idx, f)| {
                if *f == 0 {
                    Some(idx)
                } else {
                    None
                }
            })
            .unwrap();

        let mut answer = 0;
        for idx in [1000, 2000, 3000] {
            answer += decrypted[(zero_idx + idx).rem_euclid(decrypted.len())];
        }
        answer
    }

    const SAMPLE_INPUT: &[isize] = &[1, 2, -3, 3, -2, 0, 4];

    fn personal_input() -> Vec<isize> {
        include_str!("./input/day20.txt")
            .lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}
