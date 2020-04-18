use super::{LineReader, LineWriter};
use std::io::{BufRead, Lines, StdinLock, StdoutLock, Write};

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
