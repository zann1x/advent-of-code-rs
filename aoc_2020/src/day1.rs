use std::collections::HashSet;
use std::error::Error;
use std::fs;

const INPUT_FILE: &str = "input/day1.txt";

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 01.1: {}",
        match solve_part_one(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    println!(
        "Day 01.2: {}",
        match solve_part_two(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    Ok(())
}

fn solve_part_one(path: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: HashSet<u64> = fs::read_to_string(&path)?
        .lines()
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();

    for i in input.iter() {
        let j = 2020 - i;
        if input.contains(&j) {
            return Ok(Some(i * j));
        }
    }

    Ok(None)
}

fn solve_part_two(path: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input_set: HashSet<u64> = fs::read_to_string(&path)?
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
                return Ok(Some(input_vec[i] * input_vec[j] * k));
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str = "input/day1-test.txt";

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 514579);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 241861950);
    }
}
