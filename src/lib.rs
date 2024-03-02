use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OpCode {
    Constant(Value),
    Return,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_fmt(format_args!("{}", num)),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Return => f.write_str("OP_RETURN"),
            OpCode::Constant(value) => f.write_fmt(format_args!("OP_CONSTANT: {}", value)),
        }
    }
}
