use std::error::Error;

use regex::Regex;

const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7.txt"));

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 07.1: {}",
        match solve_part_one(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    println!(
        "Day 07.2: {}",
        match solve_part_two(INPUT_FILE)? {
            Some(v) => v.to_string(),
            None => "no solution".to_string(),
        }
    );

    Ok(())
}

struct BagInfo {
    name: String,
    contents: Vec<(u64, String)>,
}

fn solve_part_one(file_contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let regexp_bag = Regex::new(r"^([a-z ]+) bags contain (.+)$").unwrap();
    let regexp_contained_bags = Regex::new(r"(\d) ([a-z ]+) bags?").unwrap();

    let bag_infos = input
        .iter()
        .map(|s| {
            let m = regexp_bag.captures(s).unwrap();

            BagInfo {
                name: m[1].to_string(),
                contents: regexp_contained_bags
                    .captures_iter(&m[2])
                    .map(|c| (c[1].parse().unwrap(), c[2].to_string()))
                    .collect(),
            }
        })
        .collect::<Vec<_>>();

    let mut bags_containing_shiny_gold = Vec::new();
    let mut searched_bag_name = "shiny gold";
    let mut i = 0;
    loop {
        let new_items_to_check = bag_infos
            .iter()
            .filter(|b| {
                b.contents.iter().any(|(_, name)| name == searched_bag_name)
                    && !bags_containing_shiny_gold.contains(&b.name.as_str())
            })
            .map(|b| b.name.as_str())
            .collect::<Vec<_>>();
        bags_containing_shiny_gold.extend_from_slice(&new_items_to_check);

        match bags_containing_shiny_gold.get(i) {
            Some(v) => searched_bag_name = v,
            None => break,
        }
        i += 1;
    }

    Ok(Some(bags_containing_shiny_gold.len() as u64))
}

fn count_bags(bag_infos: &Vec<BagInfo>, name: &str) -> u64 {
    1 + bag_infos
        .iter()
        .filter(|b| b.name == name)
        .collect::<Vec<_>>()
        .get(0)
        .unwrap()
        .contents
        .iter()
        .map(|(count, name)| count_bags(bag_infos, name.as_str()) * count)
        .sum::<u64>()
}

fn solve_part_two(file_contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let regexp_bag = Regex::new(r"^([a-z ]+) bags contain (.+)$").unwrap();
    let regexp_contained_bags = Regex::new(r"(\d) ([a-z ]+) bags?").unwrap();

    let bag_infos = input
        .iter()
        .map(|s| {
            let m = regexp_bag.captures(s).unwrap();

            BagInfo {
                name: m[1].to_string(),
                contents: regexp_contained_bags
                    .captures_iter(&m[2])
                    .map(|c| (c[1].parse().unwrap(), c[2].to_string()))
                    .collect(),
            }
        })
        .collect::<Vec<_>>();

    Ok(Some(count_bags(&bag_infos, "shiny gold") - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE_1: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7a-test.txt"));
    const TEST_INPUT_FILE_2_1: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7a-test.txt"));
    const TEST_INPUT_FILE_2_2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7b-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE_1).unwrap().unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn part_two_with_first_test_data() {
        let result = solve_part_two(TEST_INPUT_FILE_2_1).unwrap().unwrap();
        assert_eq!(result, 32);
    }

    #[test]
    fn part_two_with_second_test_data() {
        let result = solve_part_two(TEST_INPUT_FILE_2_2).unwrap().unwrap();
        assert_eq!(result, 126);
    }
}
