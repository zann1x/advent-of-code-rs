use anyhow::{bail, Result};
use aoc_2023::{day1, day2, day3, Solve};
use clap::{value_parser, Parser};

#[derive(Parser)]
struct Cli {
    /// Day to run (valid values are between 1 and 25).
    #[arg(value_parser = value_parser!(i32).range(1..=25))]
    day: i32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.day {
        1 => day1::Solver.solve(),
        2 => day2::Solver.solve(),
        3 => day3::Solver.solve(),
        day @ 4..=25 => bail!("day {day} is not implemented yet"),
        day => bail!("invalid day {day}, valid values are 1..=25"),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
