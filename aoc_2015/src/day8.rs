use lazy_static::lazy_static;
use regex::Regex;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day8.txt"));

pub fn solve() {
    println!("Day 08.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 08.2: {}", solve_part_two(INPUT_FILE));
}

fn solve_part_one(input: &str) -> usize {
    let code_char_count = input.lines().map(|s| s.chars().count()).sum::<usize>();
    let inner_char_count = input
        .lines()
        .map(count_string_value_characters)
        .sum::<usize>();

    code_char_count - inner_char_count
}

fn count_string_value_characters(s: &str) -> usize {
    lazy_static! {
        static ref REGEXP: Regex = Regex::new(r#"(\\"|\\\\|\\x[a-fA-F0-9]{2})"#).unwrap();
    };

    // Replace the matched substring with a dummy value as we don't need the concrete value
    let s = REGEXP.replace_all(s, "?");

    // Don't count the enclosing quotes
    s.len() - 2
}

fn solve_part_two(input: &str) -> usize {
    let code_char_count = input.lines().map(|s| s.chars().count()).sum::<usize>();
    let expanded_char_count = input
        .lines()
        .map(count_expanded_string_value_characters)
        .sum::<usize>();

    expanded_char_count - code_char_count
}

fn count_expanded_string_value_characters(s: &str) -> usize {
    let s = s.replace('\\', "\\\\");
    let s = s.replace('\"', "\\\"");

    // Add the enclosing quotes of a printed string
    s.len() + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day8-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE);
        assert_eq!(result, 12);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE);
        assert_eq!(result, 19);
    }
}
