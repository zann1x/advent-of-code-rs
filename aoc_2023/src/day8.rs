use std::collections::HashMap;

use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day8.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        #[cfg(test)]
        const INPUT_FILE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/day8.1-test.txt"
        ));

        let mut lines = INPUT_FILE.lines();

        let instructions = lines.next().unwrap();
        lines.next().unwrap();

        let mut nodes = HashMap::new();
        for line in lines {
            let line = line.replace(' ', "");
            let (node, elements) = line.split_once('=').unwrap();
            let elements = elements.replace('(', "");
            let elements = elements.replace(')', "");
            let (left, right) = elements.split_once(',').unwrap();

            nodes.insert(
                node.to_string(),
                NodeElements {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }

        let mut steps = 0;
        let mut position = nodes.get_key_value("AAA").unwrap();
        for instruction in instructions.chars().cycle() {
            steps += 1;
            match instruction {
                'L' => {
                    position = nodes.get_key_value(&position.1.left).unwrap();
                }
                'R' => {
                    position = nodes.get_key_value(&position.1.right).unwrap();
                }
                _ => unreachable!(),
            }

            if position.0 == "ZZZ" {
                break;
            }
        }

        steps
    }

    fn solve_part_two(&self) -> Self::Value {
        #[cfg(test)]
        const INPUT_FILE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/day8.2-test.txt"
        ));

        let mut lines = INPUT_FILE.lines();

        let instructions = lines.next().unwrap();
        lines.next().unwrap();

        let mut nodes = HashMap::new();
        for line in lines {
            let line = line.replace(' ', "");
            let (node, elements) = line.split_once('=').unwrap();
            let elements = elements.replace('(', "");
            let elements = elements.replace(')', "");
            let (left, right) = elements.split_once(',').unwrap();

            nodes.insert(
                node.to_string(),
                NodeElements {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }

        let mut steps = Vec::new();
        let mut positions: Vec<_> = nodes.iter().filter(|(k, _)| k.ends_with('A')).collect();
        for (step, instruction) in instructions.chars().cycle().enumerate() {
            match instruction {
                'L' => {
                    positions = positions
                        .iter()
                        .map(|position| nodes.get_key_value(&position.1.left).unwrap())
                        .collect();
                }
                'R' => {
                    positions = positions
                        .iter()
                        .map(|position| nodes.get_key_value(&position.1.right).unwrap())
                        .collect();
                }
                _ => unreachable!(),
            }

            positions.retain(|position| {
                if position.0.ends_with('Z') {
                    steps.push(step + 1);
                    false
                } else {
                    true
                }
            });

            if positions.is_empty() {
                break;
            }
        }

        steps.into_iter().reduce(num::integer::lcm).unwrap()
    }
}

#[derive(Debug)]
struct NodeElements {
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 2);
        assert_eq!(Solver.solve_part_two(), 6);
    }
}
