use super::{LineReader, LineWriter};
use std::io::{BufRead, Write};

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
            .expect("Failed to read line from stdin");

        self.buffer
            .trim()
            .parse()
            .expect("Input was not of the type i32")
    }
}

pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self
    }
}

impl LineWriter for StdoutWriter {
    fn write_line(&mut self, value: i32) {
        let stdout = std::io::stdout();
        let mut stdout_lock = stdout.lock();

        write!(stdout_lock, "{}\n", value).expect("Failed to write output to stdout");
    }
}
