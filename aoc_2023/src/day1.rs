use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        #[cfg(test)]
        const INPUT_FILE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/day1.1-test.txt"
        ));

        INPUT_FILE
            .lines()
            .map(|line| {
                let digits: Vec<_> = line.chars().filter(char::is_ascii_digit).collect();
                let first: usize = digits.first().unwrap().to_string().parse().unwrap();
                let last: usize = digits.last().unwrap().to_string().parse().unwrap();

                (first * 10) + last
            })
            .sum()
    }

    fn solve_part_two(&self) -> Self::Value {
        #[cfg(test)]
        const INPUT_FILE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/day1.2-test.txt"
        ));

        INPUT_FILE
            .lines()
            .map(|line| {
                // A number's text can overlap, e.g., "twoneight" for "218"
                line.replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            })
            .map(|line| {
                let digits: Vec<_> = line.chars().filter(char::is_ascii_digit).collect();
                let first: usize = digits.first().unwrap().to_string().parse().unwrap();
                let last: usize = digits.last().unwrap().to_string().parse().unwrap();

                (first * 10) + last
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 142);
        assert_eq!(Solver.solve_part_two(), 281);
    }
}
