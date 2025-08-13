use std::fmt::Display;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub trait Solve {
    type Value: Display;

    fn solve(&self) {
        println!("Part 1: {}", self.solve_part_one());
        println!("Part 2: {}", self.solve_part_two());
    }

    fn solve_part_one(&self) -> Self::Value;
    fn solve_part_two(&self) -> Self::Value;
}
