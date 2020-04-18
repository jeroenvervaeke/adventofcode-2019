pub mod stdio;
pub mod testing;

pub trait LineReader {
    fn read_line(&mut self) -> i32;
}

pub trait LineWriter {
    fn write_line(&mut self, value: i32);
}
