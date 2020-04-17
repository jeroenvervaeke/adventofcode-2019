mod io;
mod operations;
mod program;

use io::{StdinReader, StdoutWriter};
use operations::Operation;
use program::Program;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_05/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let int_code = line
        .trim()
        .split(',')
        .map(|digit_str| digit_str.parse::<i32>().expect("Invalid input file"))
        .collect();

    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = StdinReader::new(stdin_lock);

    let stdout = std::io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = StdoutWriter::new(stdout_lock);

    let mut program = Program::new(int_code, stdin_reader, stdout_writer);
    program.run()?;

    Ok(())
}
