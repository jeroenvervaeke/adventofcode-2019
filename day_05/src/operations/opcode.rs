use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};

pub struct OpCode {
    pub operation: i32,
    pub mode: OpCodeMode,
}

pub struct OpCodeMode {
    pub parameter_1: ParameterMode,
    pub parameter_2: ParameterMode,
    pub parameter_3: ParameterMode,
}

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
    parameter: i32,
) -> Result<ParameterMode, Cow<'static, str>> {
    ((op_with_mode / (1000 * 10 ^ parameter)) % 10).try_into()
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
