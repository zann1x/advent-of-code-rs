const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4.txt"));

struct PassportData {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

pub fn solve() {
    println!("Day 04.1: {}", solve_part_one(INPUT_FILE));
    println!("Day 04.2: {}", solve_part_two(INPUT_FILE));
}

fn solve_part_one(file_contents: &str) -> u64 {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let mut passport_data_strings = Vec::new();
    let mut passport_data_string = String::new();
    for line in input {
        if line.is_empty() && !passport_data_string.is_empty() {
            passport_data_strings.push(passport_data_string.clone());
            passport_data_string.clear();
            continue;
        }

        passport_data_string.push_str(&line);
        passport_data_string.push(' ');
    }
    passport_data_strings.push(passport_data_string.clone());
    passport_data_string.clear();

    let passport_data = passport_data_strings
        .iter()
        .map(|s| {
            let split = s.split_whitespace();
            let mut entry = PassportData {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            };

            for part in split {
                let p = part.split(':').collect::<Vec<_>>();
                match p[0] {
                    "byr" => entry.byr = Some(p[1].to_string()),
                    "iyr" => entry.iyr = Some(p[1].to_string()),
                    "eyr" => entry.eyr = Some(p[1].to_string()),
                    "hgt" => entry.hgt = Some(p[1].to_string()),
                    "hcl" => entry.hcl = Some(p[1].to_string()),
                    "ecl" => entry.ecl = Some(p[1].to_string()),
                    "pid" => entry.pid = Some(p[1].to_string()),
                    "cid" => entry.cid = Some(p[1].to_string()),
                    _ => {}
                }
            }

            entry
        })
        .collect::<Vec<_>>();

    passport_data
        .iter()
        .filter(|p| {
            p.byr.is_some()
                && p.iyr.is_some()
                && p.eyr.is_some()
                && p.hgt.is_some()
                && p.hcl.is_some()
                && p.ecl.is_some()
                && p.pid.is_some()
        })
        .count() as u64
}

fn solve_part_two(file_contents: &str) -> u64 {
    let input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();

    let mut passport_data_strings = Vec::new();
    let mut passport_data_string = String::new();
    for line in input {
        if line.is_empty() && !passport_data_string.is_empty() {
            passport_data_strings.push(passport_data_string.clone());
            passport_data_string.clear();
            continue;
        }

        passport_data_string.push_str(&line);
        passport_data_string.push(' ');
    }
    passport_data_strings.push(passport_data_string.clone());
    passport_data_string.clear();

    let passport_data = passport_data_strings
        .iter()
        .map(|s| {
            let split = s.split_whitespace();
            let mut entry = PassportData {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            };

            for part in split {
                let p = part.split(':').collect::<Vec<_>>();
                match p[0] {
                    "byr" => entry.byr = Some(p[1].to_string()),
                    "iyr" => entry.iyr = Some(p[1].to_string()),
                    "eyr" => entry.eyr = Some(p[1].to_string()),
                    "hgt" => entry.hgt = Some(p[1].to_string()),
                    "hcl" => entry.hcl = Some(p[1].to_string()),
                    "ecl" => entry.ecl = Some(p[1].to_string()),
                    "pid" => entry.pid = Some(p[1].to_string()),
                    "cid" => entry.cid = Some(p[1].to_string()),
                    _ => {}
                }
            }

            entry
        })
        .collect::<Vec<_>>();

    passport_data
        .iter()
        .filter(|p| {
            p.byr.is_some()
                && p.iyr.is_some()
                && p.eyr.is_some()
                && p.hgt.is_some()
                && p.hcl.is_some()
                && p.ecl.is_some()
                && p.pid.is_some()
        })
        .filter(|p| (1920..=2002).contains(&p.byr.as_ref().unwrap().parse().unwrap()))
        .filter(|p| (2010..=2020).contains(&p.iyr.as_ref().unwrap().parse().unwrap()))
        .filter(|p| (2020..=2030).contains(&p.eyr.as_ref().unwrap().parse().unwrap()))
        .filter(|p| {
            let hgt = p.hgt.as_ref().unwrap();

            (hgt.ends_with("cm")
                && hgt.len() == 5
                && (150..=193).contains(&hgt[..3].parse().unwrap()))
                || (hgt.ends_with("in")
                    && hgt.len() == 4
                    && (59..=76).contains(&hgt[..2].parse().unwrap()))
        })
        .filter(|p| {
            let hcl = p.hcl.as_ref().unwrap();

            hcl.starts_with('#')
                && hcl.len() == 7
                && hcl.as_bytes()[1..]
                    .iter()
                    .all(|c| (b'0'..=b'9').contains(c) || (b'a'..=b'f').contains(c))
        })
        .filter(|p| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(&p.ecl.as_ref().unwrap().as_str())
        })
        .filter(|p| {
            let pid = p.pid.as_ref().unwrap();

            pid.len() == 9 && pid.chars().all(|c| c.is_numeric())
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FILE_1: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4a-test.txt"));
    const TEST_INPUT_FILE_2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4b-test.txt"));

    #[test]
    fn part_one() {
        let result = solve_part_one(TEST_INPUT_FILE_1);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(TEST_INPUT_FILE_2);
        assert_eq!(result, 6);
    }
}
