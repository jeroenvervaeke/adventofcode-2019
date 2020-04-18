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
}
