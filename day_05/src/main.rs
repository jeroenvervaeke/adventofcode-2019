mod operations;
mod program;

use crate::program::Program;
use operations::Operation;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_05/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let intcode = line
        .trim()
        .split(',')
        .map(|digit_str| digit_str.parse::<i32>().expect("Invalid input file"))
        .collect();

    let mut program = Program::new(intcode);
    program.run()?;

    Ok(())
}
