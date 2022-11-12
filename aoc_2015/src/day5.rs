use std::str;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5.txt"));

pub fn solve() {
    println!("Day 05.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 05.2: {}", solve_part_two(INPUT_FILE));
}

fn is_string_nice(s: &str) -> bool {
    if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
        // disallowed substring
        return false;
    }

    let vowel_count = s.chars().fold(0, |a, c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => a + 1,
        _ => a,
    });
    if vowel_count < 3 {
        // not enough vowels
        return false;
    }

    for i in 0..s.len() - 1 {
        if s.as_bytes()[i] == s.as_bytes()[i + 1] {
            // has a double letter
            return true;
        }
    }

    // not all rules are met
    false
}

fn is_string_nice_2(s: &str) -> bool {
    let mut contains_non_overlapping_letter_pair = false;
    for i in 0..s.len() - 1 {
        let pair = str::from_utf8(&s.as_bytes()[i..=i + 1]).unwrap();
        if str::from_utf8(&s.as_bytes()[i + 2..])
            .unwrap()
            .contains(pair)
        {
            contains_non_overlapping_letter_pair = true;
            break;
        }
    }
    if !contains_non_overlapping_letter_pair {
        return false;
    }

    for i in 0..s.len() - 2 {
        if s.as_bytes()[i] == s.as_bytes()[i + 2] {
            // has a double letter with one letter in between
            return true;
        }
    }

    // not all rules are met
    false
}

fn solve_part_one(input: &str) -> usize {
    input.lines().filter(|s| is_string_nice(s)).count()
}

fn solve_part_two(input: &str) -> usize {
    input.lines().filter(|s| is_string_nice_2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ugknbfddgicrmopn", true)]
    #[test_case("aaa", true)]
    #[test_case("jchzalrnumimnmhp", false)]
    #[test_case("haegwjzuvuyypxyu", false)]
    #[test_case("dvszwmarrgswjxmb", false)]
    fn nice_string(input: &str, expected_result: bool) {
        let result = is_string_nice(input);
        assert_eq!(result, expected_result);
    }

    #[test_case("qjhvhtzxzqqjkmpb", true)]
    #[test_case("xxyxx", true)]
    #[test_case("uurcxstgmygtbstg", false)]
    #[test_case("ieodomkazucvgmuy", false)]
    fn nice_string_2(input: &str, expected_result: bool) {
        let result = is_string_nice_2(input);
        assert_eq!(result, expected_result);
    }
}
