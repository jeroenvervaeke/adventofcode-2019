use super::{OpCode, Parameter, ParameterMode, ToParameter};
use std::borrow::Cow;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add {
        addend_1: Parameter,
        addend_2: Parameter,
        destination_address: usize,
    },
    Exit,
    Input {
        destination_address: usize,
    },
    Multiply {
        factor_1: Parameter,
        factor_2: Parameter,
        destination_address: usize,
    },
    Output {
        source: Parameter,
    },
    JumpIfTrue {
        condition: Parameter,
        location: Parameter,
    },
    JumpIfFalse {
        condition: Parameter,
        location: Parameter,
    },
    LessThan {
        value_1: Parameter,
        value_2: Parameter,
        destination_address: usize,
    },
    Equals {
        value_1: Parameter,
        value_2: Parameter,
        destination_address: usize,
    },
}

impl Operation {
    pub fn op_len(&self) -> usize {
        match self {
            Operation::Add { .. }
            | Operation::Multiply { .. }
            | Operation::LessThan { .. }
            | Operation::Equals { .. } => 4,
            Operation::JumpIfTrue { .. } | Operation::JumpIfFalse { .. } => 3,
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
            ) => Self::has_destination(&mode.parameter_3, || Operation::Add {
                addend_1: addend_1.to_parameter(&mode.parameter_1),
                addend_2: addend_2.to_parameter(&mode.parameter_2),
                destination_address: *destination as usize,
            }),
            (
                OpCode {
                    operation: 2,
                    ref mode,
                },
                [factor_1, factor_2, destination, ..],
            ) => Self::has_destination(&mode.parameter_3, || Operation::Multiply {
                factor_1: factor_1.to_parameter(&mode.parameter_1),
                factor_2: factor_2.to_parameter(&mode.parameter_2),
                destination_address: *destination as usize,
            }),
            (
                OpCode {
                    operation: 3,
                    ref mode,
                },
                [destination, ..],
            ) => Self::has_destination(&mode.parameter_1, || Operation::Input {
                destination_address: *destination as usize,
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
            (
                OpCode {
                    operation: 5,
                    ref mode,
                },
                [condition, location, ..],
            ) => Ok(Operation::JumpIfTrue {
                condition: condition.to_parameter(&mode.parameter_1),
                location: location.to_parameter(&mode.parameter_2),
            }),
            (
                OpCode {
                    operation: 6,
                    ref mode,
                },
                [condition, location, ..],
            ) => Ok(Operation::JumpIfFalse {
                condition: condition.to_parameter(&mode.parameter_1),
                location: location.to_parameter(&mode.parameter_2),
            }),
            (
                OpCode {
                    operation: 7,
                    ref mode,
                },
                [value_1, value_2, destination, ..],
            ) => Self::has_destination(&mode.parameter_3, || Operation::LessThan {
                value_1: value_1.to_parameter(&mode.parameter_1),
                value_2: value_2.to_parameter(&mode.parameter_2),
                destination_address: *destination as usize,
            }),
            (
                OpCode {
                    operation: 8,
                    ref mode,
                },
                [value_1, value_2, destination, ..],
            ) => Self::has_destination(&mode.parameter_3, || Operation::Equals {
                value_1: value_1.to_parameter(&mode.parameter_1),
                value_2: value_2.to_parameter(&mode.parameter_2),
                destination_address: *destination as usize,
            }),
            (OpCode { operation: 99, .. }, [..]) => Ok(Operation::Exit),
            (unsupported_opcode, _) => Err(format!(
                "Operation '{}' is not yet supported!",
                unsupported_opcode.operation
            )
            .into()),
        }
    }

    fn has_destination<F>(
        parameter: &ParameterMode,
        creator: F,
    ) -> Result<Operation, Cow<'static, str>>
    where
        F: FnOnce() -> Operation,
    {
        if *parameter == ParameterMode::Position {
            Ok(creator())
        } else {
            Err("Immediate is an invalid mode for as a destination".into())
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
                destination_address: 4,
            })
        );
    }

    #[test]
    fn parse_add_invalid_destination() {
        let opcodes = [11101, 2, 3, 4];
        let op = Operation::from_slice(&opcodes);

        assert!(op.is_err());
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
                destination_address: 4,
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
                destination_address: 5,
            })
        );
    }

    #[test]
    fn parse_multiply_invalid_destination() {
        let opcodes = [10002, 3, 4, 5];
        let op = Operation::from_slice(&opcodes);

        assert!(op.is_err());
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
                destination_address: 5,
            })
        );
    }

    #[test]
    fn parse_input_exact() {
        let opcodes = [003, 10];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Input {
                destination_address: 10
            })
        );
    }

    #[test]
    fn parse_input_exact_invalid_testination() {
        let opcodes = [103, 10];
        let op = Operation::from_slice(&opcodes);

        assert!(op.is_err());
    }

    #[test]
    fn parse_output_trailing() {
        let opcodes = [104, 10, 2];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Output {
                source: Parameter::Value(10)
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

    #[test]
    fn parse_jump_if_true_trailing() {
        let opcodes = [105, 1, 2, 3];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::JumpIfTrue {
                condition: Parameter::Value(1),
                location: Parameter::Address(2)
            })
        );
    }

    #[test]
    fn parse_jump_if_false_trailing() {
        let opcodes = [1006, 1, 2, 3];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::JumpIfFalse {
                condition: Parameter::Address(1),
                location: Parameter::Value(2)
            })
        );
    }

    #[test]
    fn parse_less_than_exact() {
        let opcodes = [107, 1, 2, 3];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::LessThan {
                value_1: Parameter::Value(1),
                value_2: Parameter::Address(2),
                destination_address: 3
            })
        );
    }

    #[test]
    fn parse_equals_trailing() {
        let opcodes = [1008, 1, 2, 3, 4];
        let op = Operation::from_slice(&opcodes);

        assert_eq!(
            op,
            Ok(Operation::Equals {
                value_1: Parameter::Address(1),
                value_2: Parameter::Value(2),
                destination_address: 3
            })
        );
    }
}
