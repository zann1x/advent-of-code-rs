use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        let mut lines = INPUT_FILE.lines();

        let times: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(' ')
            .filter(|time| !time.is_empty())
            .map(|time| time.parse::<usize>().unwrap())
            .collect();

        let distances: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(' ')
            .filter(|distance| !distance.is_empty())
            .map(|distance| distance.parse::<usize>().unwrap())
            .collect();

        let races: Vec<_> = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect();

        races
            .iter()
            .map(|race| {
                (1..race.time)
                    .map(|charge| (race.time - charge) * charge)
                    .filter(|&distance| distance > race.distance)
                    .count()
            })
            .reduce(|acc, e| acc * e)
            .unwrap()
    }

    fn solve_part_two(&self) -> Self::Value {
        let mut lines = INPUT_FILE.lines();

        let time = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();

        let race = Race { time, distance };

        (1..race.time)
            .map(|charge| (race.time - charge) * charge)
            .filter(|&distance| distance > race.distance)
            .count()
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 288);
        assert_eq!(Solver.solve_part_two(), 71503);
    }
}
