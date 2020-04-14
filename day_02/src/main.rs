use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 2 boilerplate");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_work() {
        assert_eq!(1 + 1, 2);
    }
}
