use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_01/input.txt")?;
    let reader = BufReader::new(file);

    let solution = reader
        .lines()
        .filter_map(|line_result| line_result.ok().and_then(|line| line.parse().ok()))
        .fold(0, |acc, next| acc + calculate_fuel(next));

    println!("Sum: {}", solution);

    Ok(())
}

fn calculate_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_12() {
        assert_eq!(calculate_fuel(12), 2);
    }

    #[test]
    fn example_13() {
        assert_eq!(calculate_fuel(13), 2);
    }

    #[test]
    fn example_1969() {
        assert_eq!(calculate_fuel(1969), 654);
    }

    #[test]
    fn example_100756() {
        assert_eq!(calculate_fuel(100756), 33583);
    }
}
