use super::ParameterMode;

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Address(usize),
    Value(i32),
}

pub trait ToParameter {
    fn to_parameter(&self, mode: &ParameterMode) -> Parameter;
}

impl ToParameter for i32 {
    fn to_parameter(&self, mode: &ParameterMode) -> Parameter {
        match mode {
            ParameterMode::Position => Parameter::Address(*self as usize),
            ParameterMode::Immediate => Parameter::Value(*self),
        }
    }
}
