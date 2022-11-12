use std::collections::HashSet;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1.txt"));

pub fn solve() {
    println!("Day 01.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 01.2: {}", solve_part_two(INPUT_FILE));
}

fn solve_part_one(file_contents: &str) -> u64 {
    let input: HashSet<u64> = file_contents
        .lines()
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();

    for i in input.iter() {
        let j = 2020 - i;
        if input.contains(&j) {
            return i * j;
        }
    }

    unreachable!();
}

fn solve_part_two(file_contents: &str) -> u64 {
    let input_set: HashSet<u64> = file_contents
        .lines()
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();
    let input_vec: Vec<&u64> = input_set.iter().collect();

    for i in 0..input_vec.len() - 1 {
        for j in i + 1..input_vec.len() {
            if input_vec[i] + input_vec[j] > 2020 {
                continue;
            }

            let k = 2020 - input_vec[i] - input_vec[j];
            if input_set.contains(&k) {
                return input_vec[i] * input_vec[j] * k;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE);
        assert_eq!(result, 514579);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE);
        assert_eq!(result, 241861950);
    }
}
