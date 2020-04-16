use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod operations;

use operations::Operation;
use std::borrow::Cow;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_05/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let intcode: Vec<i32> = line
        .trim()
        .split(',')
        .map(|digit_str| digit_str.parse::<i32>().expect("Invalid input file"))
        .collect();

    let mut memory = intcode.clone();
    run_intcode(memory.as_mut_slice())?;

    Ok(())
}

fn run_intcode(memory: &mut [i32]) -> Result<(), Cow<'static, str>> {
    let mut idx = 0;

    loop {
        let op_code = Operation::from_slice(&memory[idx..])?;

        match &op_code {
            Operation::Add {
                addend_1,
                addend_2,
                destination,
            } => {
                let result = addend_1.materialize(memory) + addend_2.materialize(memory);
                destination.store(memory, result);
            }
            Operation::Multiply {
                factor_1,
                factor_2,
                destination,
            } => {
                let result = factor_1.materialize(memory) * factor_2.materialize(memory);
                destination.store(memory, result);
            }
            Operation::Exit => break,
            _ => unimplemented!(),
        }

        idx += op_code.op_len();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example_explained_in_text() {
        let mut memory = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let result = run_intcode(&mut memory);

        assert_eq!(result.is_ok(), true);
        assert_eq!(memory, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn run_example_short_1() {
        let mut memory = [1, 0, 0, 0, 99];

        let result = run_intcode(&mut memory);

        assert_eq!(result.is_ok(), true);
        assert_eq!(memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn run_example_short_2() {
        let mut memory = [2, 3, 0, 3, 99];

        let result = run_intcode(&mut memory);

        assert_eq!(result.is_ok(), true);
        assert_eq!(memory, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn run_example_short_3() {
        let mut memory = [2, 4, 4, 5, 99, 0];

        let result = run_intcode(&mut memory);

        assert_eq!(result.is_ok(), true);
        assert_eq!(memory, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn run_example_short_4() {
        let mut memory = [1, 1, 1, 4, 99, 5, 6, 0, 99];

        let result = run_intcode(&mut memory);

        assert_eq!(result.is_ok(), true);
        assert_eq!(memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
