use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day9.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = i64;

    fn solve_part_one(&self) -> Self::Value {
        #[cfg(test)]
        const INPUT_FILE: &str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day9-test.txt"));

        let lines = INPUT_FILE.lines();

        let mut extrapolations = Vec::new();
        for line in lines {
            let nums = line
                .split(' ')
                .map(std::str::FromStr::from_str)
                .collect::<Result<Vec<i64>, std::num::ParseIntError>>()
                .unwrap();

            extrapolations.push(process_levels(&nums));
        }

        extrapolations.iter().sum()
    }

    fn solve_part_two(&self) -> Self::Value {
        #[cfg(test)]
        const INPUT_FILE: &str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day9-test.txt"));

        let lines = INPUT_FILE.lines();

        let mut extrapolations = Vec::new();
        for line in lines {
            let nums = line
                .split(' ')
                .map(std::str::FromStr::from_str)
                .collect::<Result<Vec<i64>, std::num::ParseIntError>>()
                .unwrap();

            extrapolations.push(process_levels_rev(&nums));
        }

        extrapolations.iter().sum()
    }
}

fn process_level(level: &[i64]) -> Vec<i64> {
    if level.len() == 1 {
        return vec![0];
    }

    level.windows(2).map(|v| (v[1] - v[0])).collect()
}

fn process_levels(level: &[i64]) -> i64 {
    if level.iter().all(|v| *v == 0) {
        return *level.last().unwrap();
    }

    let next = process_level(level);
    level.last().unwrap() + process_levels(&next)
}

fn process_levels_rev(level: &[i64]) -> i64 {
    if level.iter().all(|v| *v == 0) {
        return *level.first().unwrap();
    }

    let next = process_level(level);
    level.first().unwrap() - process_levels_rev(&next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 114);
        assert_eq!(Solver.solve_part_two(), 2);
    }
}
