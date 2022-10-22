use std::{error::Error, fs};

const INPUT_FILE: &str = "input/day5.txt";

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 05.1: {}",
        match solve_part_one(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    println!(
        "Day 05.2: {}",
        match solve_part_two(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    Ok(())
}

fn find_position(s: &str, lower_bound: u64, upper_bound: u64) -> u64 {
    if lower_bound == upper_bound {
        return lower_bound;
    }

    let c = s.as_bytes()[0];
    match c {
        b'F' | b'L' => find_position(
            &s[1..],
            lower_bound,
            (upper_bound - lower_bound) / 2 + lower_bound,
        ),
        b'B' | b'R' => find_position(
            &s[1..],
            (upper_bound - lower_bound + 1) / 2 + lower_bound,
            upper_bound,
        ),
        _ => panic!("Unexpected character"),
    }
}

fn solve_part_one(path: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string(&path)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(input
        .iter()
        .map(|s| find_position(&s[..7], 0, 127) * 8 + find_position(&s[7..], 0, 7))
        .reduce(|a, b| a.max(b)))
}

fn solve_part_two(path: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string(&path)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    let ids = input
        .iter()
        .map(|s| find_position(&s[..7], 0, 127) * 8 + find_position(&s[7..], 0, 7))
        .collect::<Vec<u64>>();
    for i in *ids.iter().min().unwrap()..*ids.iter().max().unwrap() {
        if !ids.contains(&i) && ids.contains(&(i - 1)) && ids.contains(&(i + 1)) {
            return Ok(Some(i));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str = "input/day5-test.txt";

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 820);
    }

    #[test]
    fn part_two() {
        // NOTE: There is no given input to test a solution which returns an ID
        let result = solve_part_two(TEST_INPUT_FILE).unwrap();
        assert!(result.is_none());
    }
}
