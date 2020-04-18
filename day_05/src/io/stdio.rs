use super::{LineReader, LineWriter};
use std::io::{BufRead, Lines, Stdin, StdinLock, StdoutLock, Write};

pub struct StdinReader {
    buffer: String,
}

impl<'a> StdinReader {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(32),
        }
    }
}

impl LineReader for StdinReader {
    fn read_line(&mut self) -> i32 {
        let stdin = std::io::stdin();
        let mut lock = stdin.lock();

        self.buffer.clear();
        lock.read_line(&mut self.buffer)
            .expect("Failed to read line");

        self.buffer
            .trim()
            .parse()
            .expect("Input was not of the type i32")
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
