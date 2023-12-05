use std::{collections::HashMap, ops::Range, str::Lines};

use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        let mut lines = INPUT_FILE.lines();

        let mut seeds = Vec::new();
        let mut projections = HashMap::<ProjectionType, Vec<Mapping>>::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with("seeds:") {
                let (_, seed_values) = line.split_once(": ").unwrap();
                seeds = seed_values
                    .split(' ')
                    .map(|seed| seed.parse::<usize>().unwrap())
                    .collect();
                continue;
            }

            let projection_type = match line {
                "seed-to-soil map:" => ProjectionType::SeedToSoil,
                "soil-to-fertilizer map:" => ProjectionType::SoilToFertilizer,
                "fertilizer-to-water map:" => ProjectionType::FertilizerToWater,
                "water-to-light map:" => ProjectionType::WaterToLight,
                "light-to-temperature map:" => ProjectionType::LightToTemperature,
                "temperature-to-humidity map:" => ProjectionType::TemperatureToHumidity,
                "humidity-to-location map:" => ProjectionType::HumidityToLocation,
                _ => unreachable!("encountered unexpected line {line}"),
            };

            projections.insert(projection_type, parse_projection(&mut lines));
        }

        seeds
            .into_iter()
            .map(|seed| {
                let v = find_mapped_destination(ProjectionType::SeedToSoil, &projections, seed);
                let v = find_mapped_destination(ProjectionType::SoilToFertilizer, &projections, v);
                let v = find_mapped_destination(ProjectionType::FertilizerToWater, &projections, v);
                let v = find_mapped_destination(ProjectionType::WaterToLight, &projections, v);
                let v =
                    find_mapped_destination(ProjectionType::LightToTemperature, &projections, v);
                let v =
                    find_mapped_destination(ProjectionType::TemperatureToHumidity, &projections, v);
                find_mapped_destination(ProjectionType::HumidityToLocation, &projections, v)
            })
            .min()
            .unwrap()
    }

    fn solve_part_two(&self) -> Self::Value {
        let mut lines = INPUT_FILE.lines();

        let mut seeds = Vec::new();
        let mut projections = HashMap::<ProjectionType, Vec<Mapping>>::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with("seeds:") {
                let (_, seed_values) = line.split_once(": ").unwrap();
                let seed_values: Vec<_> = seed_values
                    .split(' ')
                    .map(|seed| seed.parse::<usize>().unwrap())
                    .collect();

                let mut split_index = 0;
                while split_index < seed_values.len() {
                    let values = seed_values.get(split_index..split_index + 2).unwrap();
                    let start = *values.first().unwrap();
                    let size = *values.last().unwrap();
                    seeds.push(start..start + size);
                    split_index += 2;
                }

                continue;
            }

            let projection_type = match line {
                "seed-to-soil map:" => ProjectionType::SeedToSoil,
                "soil-to-fertilizer map:" => ProjectionType::SoilToFertilizer,
                "fertilizer-to-water map:" => ProjectionType::FertilizerToWater,
                "water-to-light map:" => ProjectionType::WaterToLight,
                "light-to-temperature map:" => ProjectionType::LightToTemperature,
                "temperature-to-humidity map:" => ProjectionType::TemperatureToHumidity,
                "humidity-to-location map:" => ProjectionType::HumidityToLocation,
                _ => unreachable!("encountered unexpected line {line}"),
            };

            projections.insert(projection_type, parse_projection(&mut lines));
        }

        seeds
            .into_iter()
            .map(|seed_range| find_mapped_destination_in_range(&projections, seed_range))
            .min()
            .unwrap()
    }
}

fn find_mapped_destination(
    ty: ProjectionType,
    projections: &HashMap<ProjectionType, Vec<Mapping>>,
    seed: usize,
) -> usize {
    let projection = projections.get(&ty).unwrap();
    if let Some(index) = projection
        .iter()
        .position(|mapping| mapping.source_range.contains(&seed))
    {
        let mapping = projection.get(index).unwrap();
        mapping.destination_range.start + (seed - mapping.source_range.start)
    } else {
        seed
    }
}

fn find_mapped_destination_in_range(
    projections: &HashMap<ProjectionType, Vec<Mapping>>,
    seed_range: Range<usize>,
) -> usize {
    seed_range
        .map(|seed| {
            let v = find_mapped_destination(ProjectionType::SeedToSoil, projections, seed);
            let v = find_mapped_destination(ProjectionType::SoilToFertilizer, projections, v);
            let v = find_mapped_destination(ProjectionType::FertilizerToWater, projections, v);
            let v = find_mapped_destination(ProjectionType::WaterToLight, projections, v);
            let v = find_mapped_destination(ProjectionType::LightToTemperature, projections, v);
            let v = find_mapped_destination(ProjectionType::TemperatureToHumidity, projections, v);
            find_mapped_destination(ProjectionType::HumidityToLocation, projections, v)
        })
        .min()
        .unwrap()
}

fn parse_projection(lines: &mut Lines) -> Vec<Mapping> {
    let mut mappings = Vec::new();
    loop {
        let Some(line) = lines.next() else {
            // EOF
            break;
        };
        if line.is_empty() {
            // Block is finished
            break;
        }

        let mut parts = line.split(' ');
        let destination = parts.next().unwrap().parse::<usize>().unwrap();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let size = parts.next().unwrap().parse::<usize>().unwrap();

        mappings.push(Mapping {
            source_range: source..source + size,
            destination_range: destination..destination + size,
        });
    }

    mappings
}

#[derive(Debug)]
struct Mapping {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum ProjectionType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 35);
        assert_eq!(Solver.solve_part_two(), 46);
    }
}
