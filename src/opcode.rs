use crate::value::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OpCode {
    Constant(Value),
    Negate,
    Return,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<f64> for OpCode {
    fn from(value: f64) -> Self {
        OpCode::Constant(Value::from(value))
    }
}

impl From<Value> for OpCode {
    fn from(value: Value) -> Self {
        Self::Constant(value)
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Return => f.write_str("OP_RETURN"),
            OpCode::Constant(value) => f.write_fmt(format_args!("OP_CONSTANT: {}", value)),
            OpCode::Negate => f.write_str("OP_NEGATE"),
            OpCode::Add => f.write_str("OP_ADD"),
            OpCode::Subtract => f.write_str("OP_SUBTRACT"),
            OpCode::Multiply => f.write_str("OP_MULTIPLY"),
            OpCode::Divide => f.write_str("OP_DIVIDE"),
        }
    }
}
