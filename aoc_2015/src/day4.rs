use md5::{Digest, Md5};

const INPUT: &str = "iwrupvqb";

pub fn solve() {
    println!("Day 04.1: {}", solve_part_one(INPUT));
    println!("Day 04.2: {}", solve_part_two(INPUT));
}

fn solve_part_one(input: &str) -> u64 {
    let mut hasher = Md5::new();
    let input_bytes = input.as_bytes();

    for i in 0..=std::u64::MAX {
        hasher.update(input_bytes);
        hasher.update(i.to_string().as_bytes());

        let hash = hasher.finalize_reset();

        // The value in hash is hexadecimal which means one byte takes up two "printed" characters.
        // Therefore, we have to check the first 2.5 bytes of the array.
        // The fifth zero character can be found by checking whether only the latter "printed"
        // character in the third byte contains something.
        if hash[..2] == [0x0, 0x0] && hash[2] <= 0x0f {
            return i;
        }
    }

    unreachable!();
}

fn solve_part_two(input: &str) -> u64 {
    let mut hasher = Md5::new();
    let input_bytes = input.as_bytes();

    for i in 0..=std::u64::MAX {
        hasher.update(input_bytes);
        hasher.update(i.to_string().as_bytes());

        let hash = hasher.finalize_reset();

        // The value in hash is hexadecimal which means one byte takes up two "printed" characters.
        // Therefore, we have to check the first 3 bytes of the array.
        if hash[..=2] == [0x0, 0x0, 0x0] {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdef", 609043)]
    #[test_case("pqrstuv", 1048970)]
    fn part_one(input: &str, expected_result: u64) {
        let result = solve_part_one(input);
        assert_eq!(result, expected_result);
    }
}
