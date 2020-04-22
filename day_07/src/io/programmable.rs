use super::{LineReader, LineWriter};

pub struct ProgrammableInput {
    current: usize,
    inputs: Vec<i32>,
}

impl ProgrammableInput {
    pub fn new(inputs: Vec<i32>) -> Self {
        Self { current: 0, inputs }
    }
}

impl LineReader for ProgrammableInput {
    fn read_line(&mut self) -> i32 {
        let value = *self
            .inputs
            .get(self.current)
            .expect("Attempted to read past end");
        self.current += 1;

        value
    }
}

pub struct ProgrammableOutput {
    output: Vec<i32>,
}

impl ProgrammableOutput {
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }

    pub fn output(self) -> Vec<i32> {
        self.output
    }
}

impl LineWriter for ProgrammableOutput {
    fn write_line(&mut self, value: i32) {
        self.output.push(value)
    }
}
