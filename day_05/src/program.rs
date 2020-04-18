use super::{
    io::{LineReader, LineWriter},
    operations::Parameter,
    Operation,
};
use std::borrow::Cow;

pub struct Program<Input, Output>
where
    Input: LineReader,
    Output: LineWriter,
{
    memory: Vec<i32>,
    input: Input,
    output: Output,
}

impl<Input, Output> Program<Input, Output>
where
    Input: LineReader,
    Output: LineWriter,
{
    pub fn new(memory: Vec<i32>, input: Input, output: Output) -> Self {
        Self {
            input,
            memory,
            output,
        }
    }

    pub fn run(&mut self) -> Result<(), Cow<'static, str>> {
        let mut idx = 0;

        loop {
            let op_code = Operation::from_slice(&self.memory[idx..])?;

            match &op_code {
                Operation::Add {
                    addend_1,
                    addend_2,
                    destination_address,
                } => {
                    let result = self.load(addend_1) + self.load(addend_2);
                    self.store(destination_address, result);
                }
                Operation::Multiply {
                    factor_1,
                    factor_2,
                    destination_address,
                } => {
                    let result = self.load(factor_1) * self.load(factor_2);
                    self.store(destination_address, result);
                }
                Operation::Exit => break,
                Operation::Input {
                    destination_address,
                } => {
                    let value = self.input.read_line();
                    self.store(destination_address, value)
                }
                Operation::Output { source } => {
                    let value = self.load(source);
                    self.output.write_line(value);
                }
                Operation::JumpIfTrue {
                    condition,
                    location,
                } => {
                    let value = self.load(condition);
                    if value != 0 {
                        let location = self.load(location);
                        idx = location as usize;
                        continue;
                    }
                }
                Operation::JumpIfFalse {
                    condition,
                    location,
                } => {
                    let value = self.load(condition);
                    if value == 0 {
                        let location = self.load(location);
                        idx = location as usize;
                        continue;
                    }
                }
                Operation::LessThan {
                    value_1,
                    value_2,
                    destination_address,
                } => {
                    let value_1 = self.load(value_1);
                    let value_2 = self.load(value_2);

                    if value_1 < value_2 {
                        self.store(destination_address, 1)
                    } else {
                        self.store(destination_address, 0)
                    }
                }
                Operation::Equals {
                    value_1,
                    value_2,
                    destination_address,
                } => {
                    let value_1 = self.load(value_1);
                    let value_2 = self.load(value_2);

                    if value_1 == value_2 {
                        self.store(destination_address, 1)
                    } else {
                        self.store(destination_address, 0)
                    }
                }
            }

            idx += op_code.op_len();
        }

        Ok(())
    }

    fn load(&self, parameter: &Parameter) -> i32 {
        match parameter {
            Parameter::Address(idx) => self.memory[*idx],
            Parameter::Value(value) => *value,
        }
    }

    fn store(&mut self, address: &usize, value: i32) {
        self.memory[*address] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::testing::{UnitTestInput, UnitTestOutput};

    fn null_input_and_output() -> (UnitTestInput, UnitTestOutput) {
        (
            UnitTestInput::new(Vec::new()),
            UnitTestOutput::new(Vec::new()),
        )
    }

    fn run_fixed_io(program: Vec<i32>, input: Vec<i32>, output: Vec<i32>) {
        let input = UnitTestInput::new(input);
        let output = UnitTestOutput::new(output);

        let mut program = Program::new(program, input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn day_02_run_example_explained_in_text() {
        let (input, output) = null_input_and_output();
        let mut program = Program::new(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            input,
            output,
        );
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            program.memory,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn day_02_run_example_short_1() {
        let (input, output) = null_input_and_output();
        let mut program = Program::new(vec![1, 0, 0, 0, 99], input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn day_02_run_example_short_2() {
        let (input, output) = null_input_and_output();
        let mut program = Program::new(vec![2, 3, 0, 3, 99], input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn day_02_run_example_short_3() {
        let (input, output) = null_input_and_output();
        let mut program = Program::new(vec![2, 4, 4, 5, 99, 0], input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn day_02_run_example_short_4() {
        let (input, output) = null_input_and_output();
        let mut program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day_05_run_negative() {
        let (input, output) = null_input_and_output();
        let mut program = Program::new(vec![1101, 100, -1, 4, 0], input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn day_05_test_io() {
        let input = UnitTestInput::new(vec![1, 2]);
        let output = UnitTestOutput::new(vec![3]);

        let mut program = Program::new(vec![3, 0, 3, 1, 1, 0, 1, 2, 4, 2, 99], input, output);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [1, 2, 3, 1, 1, 0, 1, 2, 4, 2, 99]);
    }

    #[test]
    fn day_05_part_2_example_1_input_8() {
        run_fixed_io(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8], vec![1]);
    }

    #[test]
    fn day_05_part_2_example_1_input_9() {
        run_fixed_io(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![9], vec![0]);
    }

    #[test]
    fn day_05_part_2_example_2_input_7() {
        run_fixed_io(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![7], vec![1]);
    }

    #[test]
    fn day_05_part_2_example_2_input_9() {
        run_fixed_io(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![9], vec![0]);
    }

    #[test]
    fn day_05_part_2_example_3_input_8() {
        run_fixed_io(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8], vec![1]);
    }

    #[test]
    fn day_05_part_2_example_3_input_9() {
        run_fixed_io(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![9], vec![0]);
    }

    #[test]
    fn day_05_part_2_example_4_input_7() {
        run_fixed_io(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![7], vec![1]);
    }

    #[test]
    fn day_05_part_2_example_4_input_9() {
        run_fixed_io(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![9], vec![0]);
    }

    #[test]
    fn day_05_part_2_example_5_input_0() {
        run_fixed_io(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
            vec![0],
        );
    }

    #[test]
    fn day_05_part_2_example_5_input_5() {
        run_fixed_io(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![5],
            vec![1],
        );
    }

    #[test]
    fn day_05_part_2_example_6_input_0() {
        run_fixed_io(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![0],
            vec![0],
        );
    }

    #[test]
    fn day_05_part_2_example_6_input_5() {
        run_fixed_io(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![5],
            vec![1],
        );
    }

    #[test]
    fn day_05_part_2_example_7_input_7() {
        run_fixed_io(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![7],
            vec![999],
        );
    }

    #[test]
    fn day_05_part_2_example_7_input_8() {
        run_fixed_io(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![8],
            vec![1000],
        );
    }

    #[test]
    fn day_05_part_2_example_7_input_9() {
        run_fixed_io(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![9],
            vec![1001],
        );
    }
}
