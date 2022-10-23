use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6.txt"));

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 06.1: {}",
        match solve_part_one(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    println!(
        "Day 06.2: {}",
        match solve_part_two(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    Ok(())
}

fn solve_part_one(file_contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let mut sum = 0;
    let mut group_answers = HashSet::new();
    for line in input.iter() {
        if line.is_empty() {
            sum += group_answers.len() as u64;
            group_answers.clear();
        }

        for c in line.chars() {
            group_answers.insert(c);
        }
    }
    sum += group_answers.len() as u64;
    group_answers.clear();

    Ok(Some(sum))
}

fn solve_part_two(file_contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let mut sum: u64 = 0;
    let mut group_members = 0;
    let mut group_answers: HashMap<char, u64> = HashMap::new();
    for line in input.iter() {
        if line.is_empty() {
            sum += group_answers
                .iter()
                .filter(|p| p.1 == &group_members)
                .count() as u64;
            group_answers.clear();
            group_members = 0;

            continue;
        }

        group_members += 1;
        for c in line.chars() {
            group_answers.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    sum += group_answers
        .iter()
        .filter(|p| p.1 == &group_members)
        .count() as u64;
    group_answers.clear();

    Ok(Some(sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 6);
    }
}
