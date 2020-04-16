use super::{OpCode, Parameter, ToParameter};
use std::borrow::Cow;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add {
        addend_1: Parameter,
        addend_2: Parameter,
        destination: Parameter,
    },
    Exit,
    Input {
        destination: Parameter,
    },
    Multiply {
        factor_1: Parameter,
        factor_2: Parameter,
        destination: Parameter,
    },
    Output {
        source: Parameter,
    },
}

impl Operation {
    pub fn op_len(&self) -> usize {
        match self {
            Operation::Add { .. } | Operation::Multiply { .. } => 4,
            Operation::Input { .. } | Operation::Output { .. } => 2,
            Operation::Exit => 1,
        }
    }

    pub fn from_slice(current: &[i32]) -> Result<Operation, Cow<'static, str>> {
        match Self::split_opcode(current)? {
            (
                OpCode {
                    operation: 1,
                    ref mode,
                },
                [addend_1, addend_2, destination, ..],
            ) => Ok(Operation::Add {
                addend_1: addend_1.to_parameter(&mode.parameter_1),
                addend_2: addend_2.to_parameter(&mode.parameter_2),
                destination: destination.to_parameter(&mode.parameter_3),
            }),
            (
                OpCode {
                    operation: 2,
                    ref mode,
                },
                [factor_1, factor_2, destination, ..],
            ) => Ok(Operation::Multiply {
                factor_1: factor_1.to_parameter(&mode.parameter_1),
                factor_2: factor_2.to_parameter(&mode.parameter_2),
                destination: destination.to_parameter(&mode.parameter_3),
            }),
            (
                OpCode {
                    operation: 3,
                    ref mode,
                },
                [destination, ..],
            ) => Ok(Operation::Input {
                destination: destination.to_parameter(&mode.parameter_1),
            }),
            (
                OpCode {
                    operation: 4,
                    ref mode,
                },
                [source, ..],
            ) => Ok(Operation::Output {
                source: source.to_parameter(&mode.parameter_1),
            }),
            (OpCode { operation: 99, .. }, [..]) => Ok(Operation::Exit),
            (unsupported_opcode, _) => Err(format!(
                "Operation '{}' is not yet supported!",
                unsupported_opcode.operation
            )
            .into()),
        }
    }

    fn split_opcode(slice: &[i32]) -> Result<(OpCode, &[i32]), Cow<'static, str>> {
        if slice.len() > 0 {
            Ok((slice[0].try_into()?, &slice[1..]))
        } else {
            Err("Invalid instruction".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_exact() {
        let opcodes = [1, 2, 3, 4];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Add {
                addend_1: Parameter::Address(2),
                addend_2: Parameter::Address(3),
                destination: Parameter::Address(4)
            })
        );
    }

    #[test]
    fn parse_add_trailing() {
        let opcodes = [1, 2, 3, 4, 5, 6];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Add {
                addend_1: Parameter::Address(2),
                addend_2: Parameter::Address(3),
                destination: Parameter::Address(4)
            })
        );
    }

    #[test]
    fn parse_add_too_short() {
        let opcodes = [1, 2, 3];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(op.is_err(), true);
    }

    #[test]
    fn parse_multiply_exact() {
        let opcodes = [2, 3, 4, 5];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Multiply {
                factor_1: Parameter::Address(3),
                factor_2: Parameter::Address(4),
                destination: Parameter::Address(5)
            })
        );
    }

    #[test]
    fn parse_multiply_trailing() {
        let opcodes = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Multiply {
                factor_1: Parameter::Address(3),
                factor_2: Parameter::Address(4),
                destination: Parameter::Address(5)
            })
        );
    }

    #[test]
    fn parse_exit_exact() {
        let opcodes = [99];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(op, Ok(Operation::Exit));
    }

    #[test]
    fn parse_exit_trailing() {
        let opcodes = [99, 100];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(op, Ok(Operation::Exit));
    }
}
