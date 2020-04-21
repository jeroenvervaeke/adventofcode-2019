mod opcode;
mod operation;
mod parameter;

pub use opcode::{OpCode, OpCodeMode, ParameterMode};
pub use operation::Operation;
pub use parameter::{Parameter, ToParameter};
