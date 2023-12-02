use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        const RED_MAX: usize = 12;
        const GREEN_MAX: usize = 13;
        const BLUE_MAX: usize = 14;

        INPUT_FILE
            .lines()
            .filter_map(|line| {
                let game: Game = line.parse().unwrap();

                for cube_set in &game.cube_sets {
                    if cube_set.red > RED_MAX
                        || cube_set.green > GREEN_MAX
                        || cube_set.blue > BLUE_MAX
                    {
                        return None;
                    }
                }

                Some(game)
            })
            .map(|game| game.id)
            .sum()
    }

    fn solve_part_two(&self) -> Self::Value {
        INPUT_FILE
            .lines()
            .map(|line| {
                let game: Game = line.parse().unwrap();

                let mut cube_set_min = CubeSet::default();
                for cube_set in &game.cube_sets {
                    cube_set_min.red = cube_set_min.red.max(cube_set.red);
                    cube_set_min.green = cube_set_min.green.max(cube_set.green);
                    cube_set_min.blue = cube_set_min.blue.max(cube_set.blue);
                }

                cube_set_min
            })
            .map(|cube_set| cube_set.red * cube_set.green * cube_set.blue)
            .sum()
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl std::str::FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // E.g.: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        //       -> ["Game 1", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
        let game: Vec<_> = s.split(": ").collect();

        // E.g.: "Game 1" -> 1
        let id = game
            .first()
            .map(|s| s.split(' ').last().unwrap().parse().unwrap())
            .unwrap();

        // E.g.: "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        //       -> [["3 blue", "4 red"], ["1 red", "2 green", "6 blue"], ["2 green"]]
        let rounds = game
            .last()
            .map(|s| {
                s.split("; ")
                    .map(|s| s.split(", ").collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .unwrap();

        // E.g.: "3 blue, 4 red"
        //       -> CubeSet { blue: 3, red: 4, .. }
        let mut cube_sets = Vec::new();
        for round in rounds {
            let mut cube_set = CubeSet::default();

            // E.g.: "3 blue" -> CubeSet { blue: 3, .. }
            for cube in round {
                let mut cube_iter = cube.split(' ');
                let count: usize = cube_iter.next().unwrap().parse().unwrap();
                match cube_iter.next().unwrap() {
                    "red" => {
                        cube_set.red = count;
                    }
                    "green" => {
                        cube_set.green = count;
                    }
                    "blue" => {
                        cube_set.blue = count;
                    }
                    _ => unreachable!(),
                }
            }

            cube_sets.push(cube_set);
        }

        Ok(Self { id, cube_sets })
    }
}

#[derive(Debug, Default)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 8);
        assert_eq!(Solver.solve_part_two(), 2286);
    }
}
