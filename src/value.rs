use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Obj {
    String(String),
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::String(str) => f.write_fmt(format_args!("{}", str)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(Obj),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        matches!(self, Value::Bool(false) | Value::Nil)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => f.write_fmt(format_args!("{}", num)),
            Value::Bool(bool) => f.write_fmt(format_args!("{}", bool)),
            Value::Nil => f.write_str("nil"),
            Value::Obj(obj_type) => f.write_fmt(format_args!("{}", obj_type)),
        }
    }
}
