use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 2 boilerplate");

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Op {
    Add { idx1: u32, idx2: u32, dst: u32 },
    Multiply { idx1: u32, idx2: u32, dst: u32 },
    Exit,
}

fn parse_opcode(current: &[u32]) -> Option<Op> {
    match current {
        [1, idx1, idx2, dst, ..] => Some(Op::Add {
            idx1: *idx1,
            idx2: *idx2,
            dst: *dst,
        }),
        [2, idx1, idx2, dst, ..] => Some(Op::Multiply {
            idx1: *idx1,
            idx2: *idx2,
            dst: *dst,
        }),
        [99, ..] => Some(Op::Exit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_exact() {
        let opcodes = [1, 2, 3, 4];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Add {
                idx1: 2,
                idx2: 3,
                dst: 4
            })
        );
    }

    #[test]
    fn parse_add_trailing() {
        let opcodes = [1, 2, 3, 4, 5, 6];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Add {
                idx1: 2,
                idx2: 3,
                dst: 4
            })
        );
    }

    #[test]
    fn parse_add_too_short() {
        let opcodes = [1, 2, 3];
        let op = parse_opcode(&opcodes);

        assert_eq!(op, None);
    }

    #[test]
    fn parse_multiply_exact() {
        let opcodes = [2, 3, 4, 5];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Multiply {
                idx1: 3,
                idx2: 4,
                dst: 5
            })
        );
    }

    #[test]
    fn parse_multiply_trailing() {
        let opcodes = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let op = parse_opcode(&opcodes);

        assert_eq!(
            op,
            Some(Op::Multiply {
                idx1: 3,
                idx2: 4,
                dst: 5
            })
        );
    }

    #[test]
    fn parse_exit_exact() {
        let opcodes = [99];
        let op = parse_opcode(&opcodes);

        assert_eq!(op, Some(Op::Exit));
    }

    #[test]
    fn parse_exit_trailing() {
        let opcodes = [99, 100];
        let op = parse_opcode(&opcodes);

        assert_eq!(op, Some(Op::Exit));
    }
}
