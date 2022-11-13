use lazy_static::lazy_static;
use regex::{Regex, RegexSet};
use std::collections::HashMap;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7.txt"));

pub fn solve() {
    println!("Day 07.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 07.2: {}", solve_part_two(INPUT_FILE));
}

#[derive(Clone)]
struct Instruction<'a> {
    raw: &'a str,
    result: Option<u16>,
}

fn evaluate_circuit_for(wire: &str, circuit: &mut HashMap<&str, Instruction>) -> u16 {
    lazy_static! {
        static ref REGEXP_SET: RegexSet = RegexSet::new([
            r"^([a-z0-9]+) -> ([a-z]+)$", // assignment
            r"^([a-z0-9]+) AND ([a-z0-9]+) -> ([a-z]+)$", // bitand
            r"^([a-z0-9]+) OR ([a-z0-9]+) -> ([a-z]+)$", // bitor
            r"^NOT ([a-z0-9]+) -> ([a-z]+)$", // bitnot
            r"^([a-z0-9]+) LSHIFT ([0-9]+) -> ([a-z]+)$", // lshift
            r"^([a-z0-9]+) RSHIFT ([0-9]+) -> ([a-z]+)$", // rshift
        ])
        .unwrap();
        static ref REGEXPS: Vec<Regex> = REGEXP_SET
            .patterns()
            .iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect();
    }

    // If the value got evaluated already, we early-exit to avoid deeply nested recursion which
    // possibly results in endless loops.
    if circuit[wire].result.is_some() {
        return circuit[wire].result.unwrap();
    }

    let idx = REGEXP_SET
        .matches(circuit[wire].raw)
        .into_iter()
        // The expressions are non-overlapping, so we can just grab the first and only item
        .collect::<Vec<_>>()[0];

    let regexp = &REGEXPS[idx];

    let m = regexp.captures(circuit[wire].raw).unwrap();
    let result = match idx {
        // assignment
        0 => get_evaluated_u16_value(&m[1], circuit),

        // bitand
        1 => {
            let lhs = get_evaluated_u16_value(&m[1], circuit);
            let rhs = get_evaluated_u16_value(&m[2], circuit);

            lhs & rhs
        }

        // bitor
        2 => {
            let lhs = get_evaluated_u16_value(&m[1], circuit);
            let rhs = get_evaluated_u16_value(&m[2], circuit);

            lhs | rhs
        }

        // bitnot
        3 => !get_evaluated_u16_value(&m[1], circuit),

        // lshift
        4 => {
            let lhs = get_evaluated_u16_value(&m[1], circuit);
            let rhs = get_evaluated_u16_value(&m[2], circuit);

            lhs << rhs
        }

        // rshift
        5 => {
            let lhs = get_evaluated_u16_value(&m[1], circuit);
            let rhs = get_evaluated_u16_value(&m[2], circuit);

            lhs >> rhs
        }

        _ => unreachable!(),
    };

    // Store the evaluated value for early-exits
    circuit.get_mut(wire).unwrap().result = Some(result);

    circuit[wire].result.unwrap()
}

fn get_evaluated_u16_value(raw: &str, circuit: &mut HashMap<&str, Instruction>) -> u16 {
    if raw.chars().all(char::is_numeric) {
        raw.parse::<u16>().unwrap()
    } else {
        evaluate_circuit_for(raw, circuit)
    }
}

fn solve_part_one(input: &str) -> u16 {
    solve_one(input, "a")
}

fn solve_one(input: &str, desired_wire: &str) -> u16 {
    let mut circuit = input
        .lines()
        .map(|s| {
            let token_to_find = "-> ";
            let pos = s.rfind(token_to_find).unwrap();
            (
                &s[pos + token_to_find.len()..],
                Instruction {
                    raw: s,
                    result: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    evaluate_circuit_for(desired_wire, &mut circuit)
}

fn solve_part_two(input: &str) -> u16 {
    let mut circuit = input
        .lines()
        .map(|s| {
            let token_to_find = "-> ";
            let pos = s.rfind(token_to_find).unwrap();
            (
                &s[pos + token_to_find.len()..],
                Instruction {
                    raw: s,
                    result: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut circuit2 = circuit.clone();
    circuit2.get_mut("b").unwrap().result = Some(evaluate_circuit_for("a", &mut circuit));

    evaluate_circuit_for("a", &mut circuit2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("123 -> x", "x", 123 ; "assignment x")]
    #[test_case("456 -> y", "y", 456 ; "assignment y")]
    #[test_case("123 -> x\n456 -> y\nx AND y -> d", "d", 72 ; "bitand")]
    #[test_case("123 -> x\n456 -> y\nx OR y -> e", "e", 507 ; "bitor")]
    #[test_case("123 -> x\n456 -> y\nx LSHIFT 2 -> f", "f", 492 ; "lshift")]
    #[test_case("123 -> x\n456 -> y\ny RSHIFT 2 -> g", "g", 114 ; "rshift")]
    #[test_case("123 -> x\n456 -> y\nNOT x -> h", "h", 65412 ; "not x")]
    #[test_case("123 -> x\n456 -> y\nNOT y -> i", "i", 65079 ; "not y")]
    fn part_one(input: &str, desired_wire: &str, expected_result: u16) {
        let result = solve_one(input, desired_wire);
        assert_eq!(result, expected_result);
    }
}
