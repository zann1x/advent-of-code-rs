const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5.txt"));

pub fn solve() {
    println!("Day 05.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 05.2: {}", solve_part_two(INPUT_FILE));
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

fn solve_part_one(file_contents: &str) -> u64 {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    input
        .iter()
        .map(|s| find_position(&s[..7], 0, 127) * 8 + find_position(&s[7..], 0, 7))
        .reduce(|a, b| a.max(b))
        .unwrap()
}

fn solve_part_two(file_contents: &str) -> u64 {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let ids = input
        .iter()
        .map(|s| find_position(&s[..7], 0, 127) * 8 + find_position(&s[7..], 0, 7))
        .collect::<Vec<u64>>();
    for i in *ids.iter().min().unwrap()..*ids.iter().max().unwrap() {
        if !ids.contains(&i) && ids.contains(&(i - 1)) && ids.contains(&(i + 1)) {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE);
        assert_eq!(result, 820);
    }
}
