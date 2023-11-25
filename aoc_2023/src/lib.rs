use std::fmt::Display;

pub mod day1;

pub trait Solve {
    type Value: Display;

    fn solve(&self) {
        println!("Part 1: {}", self.solve_part_one());
        println!("Part 2: {}", self.solve_part_two());
    }

    fn solve_part_one(&self) -> Self::Value;
    fn solve_part_two(&self) -> Self::Value;
}
