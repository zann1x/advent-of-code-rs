use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, MulAssign},
};

use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        let mut line_points = Vec::new();

        for line in INPUT_FILE.lines() {
            let (_, line) = line.split_once(':').unwrap();
            let (mut wn, mut mn) = line.split_once('|').unwrap();
            wn = wn.trim();
            mn = mn.trim();

            let mut winning_numbers = HashSet::new();
            while !wn.is_empty() {
                let (num, rest) = wn.split_once(' ').unwrap_or((wn, ""));
                winning_numbers.insert(num.parse::<usize>().unwrap());
                wn = rest.trim();
            }

            let mut points = 0;
            while !mn.is_empty() {
                let (num, rest) = mn.split_once(' ').unwrap_or((mn, ""));
                let num = num.parse::<usize>().unwrap();
                mn = rest.trim();

                if winning_numbers.contains(&num) {
                    if points == 0 {
                        points.add_assign(1);
                    } else {
                        points.mul_assign(2);
                    }
                }
            }

            line_points.push(points);
        }

        line_points.into_iter().sum()
    }

    fn solve_part_two(&self) -> Self::Value {
        let mut card_pile: HashMap<usize, usize> = HashMap::new();

        for (line_number, line) in INPUT_FILE.lines().enumerate() {
            // Add the current card to the pile
            card_pile
                .entry(line_number)
                .and_modify(|v| v.add_assign(1))
                .or_insert(1);

            let (_, line) = line.split_once(':').unwrap();
            let (mut wn, mut mn) = line.split_once('|').unwrap();
            wn = wn.trim();
            mn = mn.trim();

            let mut winning_numbers = HashSet::new();
            while !wn.is_empty() {
                let (num, rest) = wn.split_once(' ').unwrap_or((wn, ""));
                winning_numbers.insert(num.parse::<usize>().unwrap());
                wn = rest.trim();
            }

            let mut next_new_card = line_number.add(1);
            while !mn.is_empty() {
                let (num, rest) = mn.split_once(' ').unwrap_or((mn, ""));
                let num = num.parse::<usize>().unwrap();
                mn = rest.trim();

                if winning_numbers.contains(&num) {
                    // Add the won card to the card pile -- as often as the current
                    // card already exists in the pile
                    for _ in 0..*card_pile.get(&line_number).unwrap() {
                        card_pile
                            .entry(next_new_card)
                            .and_modify(|v| v.add_assign(1))
                            .or_insert(1);
                    }
                    next_new_card.add_assign(1);
                }
            }
        }

        card_pile.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 13);
        assert_eq!(Solver.solve_part_two(), 30);
    }
}
