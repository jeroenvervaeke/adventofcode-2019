use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub struct OpCode {
    pub operation: i32,
    pub mode: OpCodeMode,
}

#[derive(Debug, PartialEq)]
pub struct OpCodeMode {
    pub parameter_1: ParameterMode,
    pub parameter_2: ParameterMode,
    pub parameter_3: ParameterMode,
}

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<i32> for OpCode {
    type Error = Cow<'static, str>;

    fn try_from(op_with_mode: i32) -> Result<Self, Self::Error> {
        Ok(OpCode {
            operation: op_with_mode % 100,
            mode: OpCodeMode {
                parameter_1: extract_parameter_mode(op_with_mode, 0)?,
                parameter_2: extract_parameter_mode(op_with_mode, 1)?,
                parameter_3: extract_parameter_mode(op_with_mode, 2)?,
            },
        })
    }
}

fn extract_parameter_mode(
    op_with_mode: i32,
    parameter: u32,
) -> Result<ParameterMode, Cow<'static, str>> {
    let divider = 100 * i32::pow(10, parameter);
    let int_mode = (op_with_mode / divider) % 10;
    int_mode.try_into()
}

impl TryFrom<i32> for ParameterMode {
    type Error = Cow<'static, str>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            unsupported_mode => {
                Err(format!("Unsupported parameter mode: {}", unsupported_mode).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_parameter_mode_11103() {
        let op_with_mode = 11103;

        assert_eq!(
            extract_parameter_mode(op_with_mode, 0),
            Ok(ParameterMode::Immediate)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 1),
            Ok(ParameterMode::Immediate)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 2),
            Ok(ParameterMode::Immediate)
        );
    }

    #[test]
    fn extract_parameter_mode_3() {
        let op_with_mode = 3;

        assert_eq!(
            extract_parameter_mode(op_with_mode, 0),
            Ok(ParameterMode::Position)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 1),
            Ok(ParameterMode::Position)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 2),
            Ok(ParameterMode::Position)
        );
    }

    #[test]
    fn extract_parameter_mode_10003() {
        let op_with_mode = 10003;

        assert_eq!(
            extract_parameter_mode(op_with_mode, 0),
            Ok(ParameterMode::Position)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 1),
            Ok(ParameterMode::Position)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 2),
            Ok(ParameterMode::Immediate)
        );
    }

    #[test]
    fn extract_parameter_mode_1002() {
        let op_with_mode = 1002;

        assert_eq!(
            extract_parameter_mode(op_with_mode, 0),
            Ok(ParameterMode::Position)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 1),
            Ok(ParameterMode::Immediate)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 2),
            Ok(ParameterMode::Position)
        );
    }

    #[test]
    fn extract_parameter_mode_102() {
        let op_with_mode = 102;

        assert_eq!(
            extract_parameter_mode(op_with_mode, 0),
            Ok(ParameterMode::Immediate)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 1),
            Ok(ParameterMode::Position)
        );
        assert_eq!(
            extract_parameter_mode(op_with_mode, 2),
            Ok(ParameterMode::Position)
        );
    }
}
