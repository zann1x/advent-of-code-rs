const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2.txt"));

pub fn solve() {
    println!("Day 02.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 02.2: {}", solve_part_two(INPUT_FILE));
}

struct RectangularCuboid {
    l: u64,
    w: u64,
    h: u64,
}

impl RectangularCuboid {
    fn calculate_surface_area(&self) -> u64 {
        let a = self.l * self.w;
        let b = self.w * self.h;
        let c = self.h * self.l;

        2 * a + 2 * b + 2 * c
    }

    fn calculate_smallest_area(&self) -> u64 {
        let a = self.l * self.w;
        let b = self.w * self.h;
        let c = self.h * self.l;

        *vec![a, b, c].iter().min().unwrap()
    }

    fn calculate_smallest_surface_area(&self) -> u64 {
        let a = 2 * self.l + 2 * self.w;
        let b = 2 * self.l + 2 * self.h;
        let c = 2 * self.w + 2 * self.h;

        *vec![a, b, c].iter().min().unwrap()
    }

    fn calculate_volume(&self) -> u64 {
        self.l * self.w * self.h
    }
}

fn solve_part_one(file_contents: &str) -> u64 {
    file_contents
        .lines()
        .map(|s| {
            let tokens = s.split('x').collect::<Vec<_>>();
            RectangularCuboid {
                l: tokens[0].parse().unwrap(),
                w: tokens[1].parse().unwrap(),
                h: tokens[2].parse().unwrap(),
            }
        })
        .fold(0, |a, b| {
            a + b.calculate_surface_area() + b.calculate_smallest_area()
        })
}

fn solve_part_two(file_contents: &str) -> u64 {
    file_contents
        .lines()
        .map(|s| {
            let tokens = s.split('x').collect::<Vec<_>>();
            RectangularCuboid {
                l: tokens[0].parse().unwrap(),
                w: tokens[1].parse().unwrap(),
                h: tokens[2].parse().unwrap(),
            }
        })
        .fold(0, |a, b| {
            a + b.calculate_smallest_surface_area() + b.calculate_volume()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("2x3x4", 58 ; "58 square feet")]
    #[test_case("1x1x10", 43 ; "43 square feet")]
    fn part_one(input: &str, expected_result: u64) {
        let result = solve_part_one(input);
        assert_eq!(result, expected_result);
    }

    #[test_case("2x3x4", 34 ; "34 feet")]
    #[test_case("1x1x10", 14 ; "14 feet")]
    fn part_two(input: &str, expected_result: u64) {
        let result = solve_part_two(input);
        assert_eq!(result, expected_result);
    }
}
