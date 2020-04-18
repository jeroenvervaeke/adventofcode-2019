use super::{LineReader, LineWriter};

pub struct UnitTestInput {
    current: usize,
    inputs: Vec<i32>,
}

impl UnitTestInput {
    pub fn new(inputs: Vec<i32>) -> Self {
        Self { current: 0, inputs }
    }

    pub fn assert_finished(&self) {
        assert_eq!(self.current, self.inputs.len(), "Not all input was read");
    }
}

impl LineReader for UnitTestInput {
    fn read_line(&mut self) -> i32 {
        let value = *self
            .inputs
            .get(self.current)
            .expect("Attempted to read too many times");
        self.current += 1;

        value
    }
}

pub struct UnitTestOutput {
    current: usize,
    expected_outputs: Vec<i32>,
}

impl UnitTestOutput {
    pub fn new(expected_outputs: Vec<i32>) -> Self {
        Self {
            current: 0,
            expected_outputs,
        }
    }

    pub fn assert_finished(&self) {
        assert_eq!(
            self.current,
            self.expected_outputs.len(),
            "Not all output was read"
        );
    }
}

impl LineWriter for UnitTestOutput {
    fn write_line(&mut self, value: i32) {
        let expected_value = *self
            .expected_outputs
            .get(self.current)
            .expect("Attempted to write too many times");

        assert_eq!(
            value, expected_value,
            "Invalid output, expected {:?}, got {:?}",
            expected_value, value
        );

        self.current += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_correct() {
        let mut input = UnitTestInput::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(input.read_line(), 1);
        assert_eq!(input.read_line(), 2);
        assert_eq!(input.read_line(), 3);
        assert_eq!(input.read_line(), 4);
        assert_eq!(input.read_line(), 5);
        input.assert_finished();
    }

    #[test]
    #[should_panic]
    fn read_not_all_are_read() {
        let mut input = UnitTestInput::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(input.read_line(), 1);
        assert_eq!(input.read_line(), 2);
        assert_eq!(input.read_line(), 3);
        assert_eq!(input.read_line(), 4);
        input.assert_finished();
    }

    #[test]
    #[should_panic]
    fn read_too_many_times() {
        let mut input = UnitTestInput::new(vec![1, 2, 3]);
        assert_eq!(input.read_line(), 1);
        assert_eq!(input.read_line(), 2);
        assert_eq!(input.read_line(), 3);
        assert_eq!(input.read_line(), 4);
        assert_eq!(input.read_line(), 5);
        input.assert_finished();
    }

    #[test]
    fn write_correct() {
        let mut output = UnitTestOutput::new(vec![1, 2, 3]);
        output.write_line(1);
        output.write_line(2);
        output.write_line(3);
    }

    #[test]
    #[should_panic]
    fn write_not_enough_output() {
        let mut output = UnitTestOutput::new(vec![1, 2, 3]);
        output.write_line(1);
        output.write_line(2);
        output.assert_finished();
    }

    #[test]
    #[should_panic]
    fn write_incorrect_values() {
        let mut output = UnitTestOutput::new(vec![1, 2, 3]);
        output.write_line(1);
        output.write_line(3);
        output.assert_finished();
    }
}
