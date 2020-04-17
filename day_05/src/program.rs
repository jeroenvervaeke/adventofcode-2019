use super::{operations::Parameter, Operation};
use std::borrow::Cow;

pub struct Program {
    memory: Vec<i32>,
}

impl Program {
    pub fn new(memory: Vec<i32>) -> Self {
        Self { memory }
    }

    pub fn run(&mut self) -> Result<(), Cow<'static, str>> {
        let mut idx = 0;

        loop {
            let op_code = Operation::from_slice(&self.memory[idx..])?;

            match &op_code {
                Operation::Add {
                    addend_1,
                    addend_2,
                    destination,
                } => {
                    let result = self.load(addend_1) + self.load(addend_2);
                    self.store(destination, result);
                }
                Operation::Multiply {
                    factor_1,
                    factor_2,
                    destination,
                } => {
                    let result = self.load(factor_1) * self.load(factor_2);
                    self.store(destination, result);
                }
                Operation::Exit => break,
                _ => unimplemented!(),
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

    fn store(&mut self, parameter: &Parameter, value: i32) {
        match parameter {
            Parameter::Address(idx) => self.memory[*idx] = value,
            Parameter::Value(_) => { /* NOP, might change this to an error later */ }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_run_example_explained_in_text() {
        let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(
            program.memory,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn day_02_run_example_short_1() {
        let mut program = Program::new(vec![1, 0, 0, 0, 99]);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn day_02_run_example_short_2() {
        let mut program = Program::new(vec![2, 3, 0, 3, 99]);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn day_02_run_example_short_3() {
        let mut program = Program::new(vec![2, 4, 4, 5, 99, 0]);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn day_02_run_example_short_4() {
        let mut program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        let result = program.run();

        assert_eq!(result.is_ok(), true);
        assert_eq!(program.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
