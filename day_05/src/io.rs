use std::io::{BufRead, Lines, StdinLock, StdoutLock, Write};

pub trait LineReader {
    fn read_line(&mut self) -> i32;
}

pub trait LineWriter {
    fn write_line(&mut self, value: i32);
}

pub struct StdinReader<'a> {
    iterator: Lines<StdinLock<'a>>,
}

impl<'a> StdinReader<'a> {
    pub fn new(stdin_lock: StdinLock<'a>) -> Self {
        let iterator = stdin_lock.lines();
        Self { iterator }
    }
}

impl<'a> LineReader for StdinReader<'a> {
    fn read_line(&mut self) -> i32 {
        // We don't care about error handling for now
        self.iterator.next().unwrap().unwrap().parse().unwrap()
    }
}

pub struct StdoutWriter<'a> {
    stdout_lock: StdoutLock<'a>,
}

impl<'a> StdoutWriter<'a> {
    pub fn new(stdout_lock: StdoutLock<'a>) -> Self {
        Self { stdout_lock }
    }
}

impl<'a> LineWriter for StdoutWriter<'a> {
    fn write_line(&mut self, value: i32) {
        self.stdout_lock
            .write(format!("{}\n", value).as_bytes())
            .unwrap();
    }
}

pub struct UnitTestInput {
    current: usize,
    inputs: Vec<i32>,
}

impl UnitTestInput {
    pub fn new(inputs: Vec<i32>) -> Self {
        Self { current: 0, inputs }
    }
}

impl LineReader for UnitTestInput {
    fn read_line(&mut self) -> i32 {
        let value = self.inputs[self.current];
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
}

impl LineWriter for UnitTestOutput {
    fn write_line(&mut self, value: i32) {
        assert_eq!(value, self.expected_outputs[self.current]);
        self.current += 1;
    }
}
