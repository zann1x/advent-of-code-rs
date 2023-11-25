use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = i32;

    fn solve_part_one(&self) -> Self::Value {
        let _ = INPUT_FILE;
        todo!()
    }

    fn solve_part_two(&self) -> Self::Value {
        let _ = INPUT_FILE;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 0);
        assert_eq!(Solver.solve_part_two(), 0);
    }
}
