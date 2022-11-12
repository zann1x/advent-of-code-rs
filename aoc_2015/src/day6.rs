use regex::Regex;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6.txt"));
const WIDTH: usize = 1000;

pub fn solve() {
    println!("Day 06.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 06.2: {}", solve_part_two(INPUT_FILE));
}

fn solve_part_one(input: &str) -> u64 {
    let mut lights = vec![false; WIDTH * WIDTH];

    let regexp = Regex::new(r"^([a-z ]+) (\d+,\d+) through (\d+,\d+)$").unwrap();
    for line in input.lines() {
        let m = regexp.captures(line).unwrap();

        let instruction = &m[1];
        let (x1, y1) = m[2].split_once(',').unwrap();
        let x1 = x1.parse::<usize>().unwrap();
        let y1 = y1.parse::<usize>().unwrap();
        let (x2, y2) = m[3].split_once(',').unwrap();
        let x2 = x2.parse::<usize>().unwrap();
        let y2 = y2.parse::<usize>().unwrap();

        for x in x1..=x2 {
            for y in y1..=y2 {
                match instruction {
                    "toggle" => lights[y * WIDTH + x] = !lights[y * WIDTH + x],
                    "turn on" => lights[y * WIDTH + x] = true,
                    "turn off" => lights[y * WIDTH + x] = false,
                    _ => {}
                }
            }
        }
    }

    lights.iter().fold(0, |a, b| match b {
        true => a + 1,
        false => a,
    })
}

fn solve_part_two(input: &str) -> u64 {
    let mut lights = vec![0; WIDTH * WIDTH];

    let regexp = Regex::new(r"^([a-z ]+) (\d+,\d+) through (\d+,\d+)$").unwrap();
    for line in input.lines() {
        let m = regexp.captures(line).unwrap();

        let instruction = &m[1];
        let (x1, y1) = m[2].split_once(',').unwrap();
        let x1 = x1.parse::<usize>().unwrap();
        let y1 = y1.parse::<usize>().unwrap();
        let (x2, y2) = m[3].split_once(',').unwrap();
        let x2 = x2.parse::<usize>().unwrap();
        let y2 = y2.parse::<usize>().unwrap();

        for x in x1..=x2 {
            for y in y1..=y2 {
                match instruction {
                    "toggle" => lights[y * WIDTH + x] += 2,
                    "turn on" => lights[y * WIDTH + x] += 1,
                    "turn off" => {
                        if lights[y * WIDTH + x] > 0 {
                            lights[y * WIDTH + x] -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    lights.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("turn on 0,0 through 999,999", 1_000_000)]
    #[test_case("toggle 0,0 through 999,0", 1000)]
    #[test_case("turn off 499,499 through 500,500", 0)]
    fn part_one(input: &str, expected_result: u64) {
        let result = solve_part_one(input);
        assert_eq!(result, expected_result);
    }

    #[test_case("turn on 0,0 through 0,0", 1)]
    #[test_case("toggle 0,0 through 999,999", 2_000_000)]
    fn part_two(input: &str, expected_result: u64) {
        let result = solve_part_two(input);
        assert_eq!(result, expected_result);
    }
}
