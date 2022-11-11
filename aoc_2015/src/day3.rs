use std::collections::HashSet;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3.txt"));

pub fn solve() {
    println!("Day 03.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 03.2: {}", solve_part_two(INPUT_FILE));
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
}

struct Route {
    current_position: Position,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Route {
    fn new() -> Self {
        Self {
            current_position: Position::new(),
        }
    }

    fn move_in_direction(&mut self, direction: char) {
        match direction {
            '^' => {
                self.current_position.y += 1;
            }
            'v' => {
                self.current_position.y -= 1;
            }
            '>' => {
                self.current_position.x += 1;
            }
            '<' => {
                self.current_position.x -= 1;
            }
            _ => {}
        };
    }
}

fn solve_part_one(file_contents: &str) -> usize {
    let mut route = Route::new();
    let mut visited_locations = HashSet::new();
    visited_locations.insert(Position::new());

    for c in file_contents.chars() {
        route.move_in_direction(c);
        visited_locations.insert(route.current_position.clone());
    }

    visited_locations.len()
}

enum DeliveryTurn {
    Santa,
    RoboSanta,
}

fn solve_part_two(file_contents: &str) -> usize {
    let mut route_santa = Route::new();
    let mut route_robo_santa = Route::new();
    let mut visited_locations = HashSet::new();
    visited_locations.insert(Position::new());
    let mut delivery_turn = DeliveryTurn::Santa;

    for c in file_contents.chars() {
        match delivery_turn {
            DeliveryTurn::Santa => {
                route_santa.move_in_direction(c);
                visited_locations.insert(route_santa.current_position.clone());
                delivery_turn = DeliveryTurn::RoboSanta;
            }
            DeliveryTurn::RoboSanta => {
                route_robo_santa.move_in_direction(c);
                visited_locations.insert(route_robo_santa.current_position.clone());
                delivery_turn = DeliveryTurn::Santa;
            }
        }
    }

    visited_locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(">", 2 ; "2 houses - a")]
    #[test_case("^>v<", 4 ; "4 houses")]
    #[test_case("^v^v^v^v^v", 2 ; "2 houses - b")]
    fn part_one(input: &str, expected_result: usize) {
        let result = solve_part_one(input);
        assert_eq!(result, expected_result);
    }

    #[test_case("^v", 3 ; "3 houses - a")]
    #[test_case("^>v<", 3 ; "3 houses - 1")]
    #[test_case("^v^v^v^v^v", 11 ; "11 houses")]
    fn part_two(input: &str, expected_result: usize) {
        let result = solve_part_two(input);
        assert_eq!(result, expected_result);
    }
}
