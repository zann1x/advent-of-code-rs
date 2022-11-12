use regex::Regex;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2.txt"));

struct InputLine {
    lower_bound: u32,
    upper_bound: u32,
    character: char,
    password: String,
}

pub fn solve() {
    println!("Day 02.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 02.2: {}", solve_part_two(INPUT_FILE));
}

fn solve_part_one(file_contents: &str) -> u64 {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let mut parsed_input = Vec::<InputLine>::new();
    let regexp = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)$").unwrap();
    for line in input.iter() {
        let m = regexp.captures(line);
        if m.is_none() {
            continue;
        }
        let m = m.unwrap();

        parsed_input.push(InputLine {
            lower_bound: m[1].parse().unwrap(),
            upper_bound: m[2].parse().unwrap(),
            character: m[3].parse().unwrap(),
            password: m[4].parse().unwrap(),
        });
    }

    let valid_password_count = parsed_input
        .iter()
        .filter(|v| {
            (v.lower_bound..=v.upper_bound)
                .contains(&(v.password.chars().filter(|c| c == &v.character).count() as u32))
        })
        .count();

    valid_password_count as u64
}

fn solve_part_two(file_contents: &str) -> u64 {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let mut parsed_input = Vec::<InputLine>::new();
    let regexp = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)$").unwrap();
    for line in input.iter() {
        let m = regexp.captures(line);
        if m.is_none() {
            continue;
        }
        let m = m.unwrap();

        parsed_input.push(InputLine {
            lower_bound: m[1].parse().unwrap(),
            upper_bound: m[2].parse().unwrap(),
            character: m[3].parse().unwrap(),
            password: m[4].parse().unwrap(),
        });
    }

    let valid_password_count = parsed_input
        .iter()
        .filter(|v| v.password.len() >= v.upper_bound as usize)
        .filter(|v| {
            (v.password.as_bytes()[v.lower_bound as usize - 1] == v.character as u8
                && v.password.as_bytes()[v.upper_bound as usize - 1] != v.character as u8)
                || (v.password.as_bytes()[v.lower_bound as usize - 1] != v.character as u8
                    && v.password.as_bytes()[v.upper_bound as usize - 1] == v.character as u8)
        })
        .count();

    valid_password_count as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE);
        assert_eq!(result, 1);
    }
}
