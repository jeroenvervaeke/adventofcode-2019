use super::ParameterMode;

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Address(usize),
    Value(i32),
}

impl Parameter {
    pub fn materialize(&self, memory: &[i32]) -> i32 {
        match self {
            Parameter::Address(idx) => memory[*idx],
            Parameter::Value(value) => *value,
        }
    }

    pub fn store(&self, memory: &mut [i32], value: i32) {
        match self {
            Parameter::Address(idx) => memory[*idx] = value,
            Parameter::Value(_) => { /* NOP, might change this to an error later */ }
        }
    }
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
