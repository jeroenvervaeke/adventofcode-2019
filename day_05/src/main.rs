use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Debug, PartialEq)]
enum Op {
    Add {
        idx1: usize,
        idx2: usize,
        dst: usize,
    },
    Multiply {
        idx1: usize,
        idx2: usize,
        dst: usize,
    },
    Exit,
}

impl Op {
    fn op_len(&self) -> usize {
        match self {
            Op::Add { .. } | Op::Multiply { .. } => 4,
            Op::Exit => 1,
        }
    }
}

fn parse_opcode(current: &[i32]) -> Option<Op> {
    match current {
        [1, idx1, idx2, dst, ..] => Some(Op::Add {
            idx1: *idx1 as usize,
            idx2: *idx2 as usize,
            dst: *dst as usize,
        }),
        [2, idx1, idx2, dst, ..] => Some(Op::Multiply {
            idx1: *idx1 as usize,
            idx2: *idx2 as usize,
            dst: *dst as usize,
        }),
        [99, ..] => Some(Op::Exit),
        _ => None,
    }
}

fn run_intcode(memory: &mut [i32]) -> Result<(), &str> {
    let mut idx = 0;

    loop {
        let op_code = parse_opcode(&memory[idx..]).ok_or("Invalid OP code")?;

        match op_code {
            Op::Add { idx1, idx2, dst } => memory[dst] = memory[idx1] + memory[idx2],
            Op::Multiply { idx1, idx2, dst } => memory[dst] = memory[idx1] * memory[idx2],
            Op::Exit => break,
        }

        idx += op_code.op_len();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_exact() {
        let opcodes = [1, 2, 3, 4];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Add {
                idx1: 2,
                idx2: 3,
                dst: 4
            })
        );
    }

    #[test]
    fn parse_add_trailing() {
        let opcodes = [1, 2, 3, 4, 5, 6];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Add {
                idx1: 2,
                idx2: 3,
                dst: 4
            })
        );
    }

    #[test]
    fn parse_add_too_short() {
        let opcodes = [1, 2, 3];
        let op = parse_opcode(&opcodes);

        assert_eq!(op, None);
    }

    #[test]
    fn parse_multiply_exact() {
        let opcodes = [2, 3, 4, 5];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Multiply {
                idx1: 3,
                idx2: 4,
                dst: 5
            })
        );
    }

    #[test]
    fn parse_multiply_trailing() {
        let opcodes = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Multiply {
                idx1: 3,
                idx2: 4,
                dst: 5
            })
        );
    }

    #[test]
    fn parse_exit_exact() {
        let opcodes = [99];
        let op = parse_opcode(&opcodes);

        assert_eq!(op, Some(Op::Exit));
    }

    #[test]
    fn parse_exit_trailing() {
        let opcodes = [99, 100];
        let op = parse_opcode(&opcodes);

        assert_eq!(op, Some(Op::Exit));
    }

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
