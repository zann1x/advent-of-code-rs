use std::error::Error;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3.txt"));

struct Traversion {
    right: usize,
    down: usize,
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 03.1: {}",
        match solve_part_one(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    println!(
        "Day 03.2: {}",
        match solve_part_two(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    Ok(())
}

fn solve_part_one(file_contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let map_width = input[0].len();
    let traversion = Traversion { right: 3, down: 1 };

    let mut tree_hit_count = 0;
    let mut x = 0;
    let mut y = 0;
    while y * map_width < (input.len() - 1) * map_width {
        x = (x + traversion.right) % map_width;
        y += traversion.down;

        if input[y].as_bytes()[x] == b'#' {
            tree_hit_count += 1;
        }
    }

    Ok(Some(tree_hit_count))
}

fn solve_part_two(file_contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let map_width = input[0].len();
    let traversions = vec![
        Traversion { right: 1, down: 1 },
        Traversion { right: 3, down: 1 },
        Traversion { right: 5, down: 1 },
        Traversion { right: 7, down: 1 },
        Traversion { right: 1, down: 2 },
    ];

    let mut tree_hit_counts = Vec::<u64>::new();

    for traversion in traversions.iter() {
        let mut tree_hit_count = 0;

        let mut x = 0;
        let mut y = 0;
        while y * map_width < (input.len() - 1) * map_width {
            x = (x + traversion.right) % map_width;
            y += traversion.down;

            if input[y].as_bytes()[x] == b'#' {
                tree_hit_count += 1;
            }
        }

        tree_hit_counts.push(tree_hit_count);
    }

    Ok(tree_hit_counts.into_iter().reduce(|a, b| a * b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE).unwrap().unwrap();
        assert_eq!(result, 336);
    }
}
