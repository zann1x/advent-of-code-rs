const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1.txt"));

pub fn solve() {
    println!("Day 01.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 01.2: {}", solve_part_two(INPUT_FILE));
}

fn solve_part_one(file_contents: &str) -> i64 {
    file_contents.chars().fold(0, |sum, c| match c {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => sum,
    })
}

fn solve_part_two(file_contents: &str) -> i64 {
    let mut sum = 0;
    let mut i = 0_i64;
    for c in file_contents.chars() {
        i += 1;
        sum = match c {
            '(' => sum + 1,
            ')' => sum - 1,
            _ => sum,
        };

        if sum < 0 {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("(())", 0 ; "floor 0 (a)")]
    #[test_case("()()", 0 ; "floor 0 (b)")]
    #[test_case("(((", 3 ; "floor 3 (a)")]
    #[test_case("(()(()(", 3 ; "floor 3 (b)")]
    #[test_case("))(((((", 3 ; "floor 3 (c)")]
    #[test_case("())", -1 ; "basement 1 (a)")]
    #[test_case("))(", -1 ; "basement 1 (b)")]
    #[test_case(")))", -3 ; "basement 3 (a)")]
    #[test_case(")())())", -3 ; "basement 3 (b)")]
    fn part_one(input: &str, expected_result: i64) {
        let result = solve_part_one(input);
        assert_eq!(result, expected_result);
    }

    #[test_case(")", 1 ; "step 1")]
    #[test_case("()())", 5 ; "step 5")]
    fn part_two(input: &str, expected_result: i64) {
        let result = solve_part_two(input);
        assert_eq!(result, expected_result);
    }
}
