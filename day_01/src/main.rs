use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

fn main() -> Result<(), Box<dyn Error>> {
    let solution_part1 = get_lines()?.fold(0, |acc, next| acc + calculate_fuel_part1(next));
    println!("Part 1: {}", solution_part1);

    let solution_part2 = get_lines()?.fold(0, |acc, next| acc + calculate_fuel_part2(next));
    println!("Part 2: {}", solution_part2);

    Ok(())
}

fn get_lines() -> IOResult<impl Iterator<Item = i32>> {
    let file = File::open("day_01/input.txt")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(|line_result| line_result.ok().and_then(|line| line.parse().ok())))
}

fn calculate_fuel_part1(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel_part2(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;

    if fuel <= 0 {
        0
    } else {
        fuel + calculate_fuel_part2(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_12() {
        assert_eq!(calculate_fuel_part1(12), 2);
    }

    #[test]
    fn example_part1_13() {
        assert_eq!(calculate_fuel_part1(13), 2);
    }

    #[test]
    fn example_part1_1969() {
        assert_eq!(calculate_fuel_part1(1969), 654);
    }

    #[test]
    fn example_part1_100756() {
        assert_eq!(calculate_fuel_part1(100756), 33583);
    }

    #[test]
    fn example_part2_14() {
        assert_eq!(calculate_fuel_part2(14), 2);
    }

    #[test]
    fn example_part2_1969() {
        assert_eq!(calculate_fuel_part2(1969), 966);
    }

    #[test]
    fn example_part2_100756() {
        assert_eq!(calculate_fuel_part2(100756), 50346);
    }
}
