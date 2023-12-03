use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        for (line_number, line) in INPUT_FILE.lines().enumerate() {
            let mut number = String::new();

            for (index, c) in line.char_indices() {
                if c.is_ascii_digit() {
                    number.push(c);
                } else {
                    if is_symbol(c) {
                        symbols.push(Symbol {
                            value: c,
                            line_number,
                            index,
                        });
                    }

                    if !number.is_empty() {
                        numbers.push(Number {
                            value: number.parse().unwrap(),
                            line_number,
                            start: index - number.len(),
                            end: index - 1,
                        });
                    }

                    number.clear();
                }

                if index == line.chars().count() - 1 && !number.is_empty() {
                    numbers.push(Number {
                        value: number.parse().unwrap(),
                        line_number,
                        start: index - number.len(),
                        end: index,
                    });
                }
            }
        }

        numbers
            .into_iter()
            .filter(|number| {
                symbols
                    .iter()
                    .filter(|symbol| {
                        symbol.line_number >= number.line_number.saturating_sub(1)
                            && symbol.line_number <= number.line_number.saturating_add(1)
                            && symbol.index >= number.start.saturating_sub(1)
                            && symbol.index <= number.end.saturating_add(1)
                    })
                    .count()
                    > 0
            })
            .map(|number| number.value)
            .reduce(|acc, e| acc + e)
            .unwrap()
    }

    fn solve_part_two(&self) -> Self::Value {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        for (line_number, line) in INPUT_FILE.lines().enumerate() {
            let mut number = String::new();

            for (index, c) in line.char_indices() {
                if c.is_ascii_digit() {
                    number.push(c);
                } else {
                    if is_symbol(c) {
                        symbols.push(Symbol {
                            value: c,
                            line_number,
                            index,
                        });
                    }

                    if !number.is_empty() {
                        numbers.push(Number {
                            value: number.parse().unwrap(),
                            line_number,
                            start: index - number.len(),
                            end: index - 1,
                        });
                    }

                    number.clear();
                }

                if index == line.chars().count() - 1 && !number.is_empty() {
                    numbers.push(Number {
                        value: number.parse().unwrap(),
                        line_number,
                        start: index - number.len(),
                        end: index,
                    });
                }
            }
        }

        symbols
            .into_iter()
            .filter(|symbol| symbol.value == '*')
            .map(|symbol| {
                let numbers: Vec<_> = numbers
                    .iter()
                    .filter(|number| {
                        number.line_number >= symbol.line_number.saturating_sub(1)
                            && number.line_number <= symbol.line_number.saturating_add(1)
                            && symbol.index >= number.start.saturating_sub(1)
                            && symbol.index <= number.end.saturating_add(1)
                    })
                    .collect();

                if numbers.len() == 2 {
                    numbers
                        .into_iter()
                        .map(|number| number.value)
                        .reduce(|acc, e| acc * e)
                        .unwrap()
                } else {
                    0
                }
            })
            .reduce(|acc, e| acc + e)
            .unwrap()
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && c.is_ascii_punctuation()
}

#[derive(Debug)]
struct Number {
    value: usize,
    line_number: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    line_number: usize,
    index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 4361);
        assert_eq!(Solver.solve_part_two(), 467835);
    }
}
