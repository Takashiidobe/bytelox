use std::{collections::HashMap, fmt};

use crate::{
    compiler::Compiler,
    opcode::OpCode,
    value::{Obj, Value},
};

#[derive(Debug, Clone)]
pub enum VMError {
    CompileTime,
    Runtime,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::CompileTime => write!(f, "compile time error"),
            VMError::Runtime => write!(f, "runtime error"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct VM {
    pub chunks: Vec<OpCode>,
    pub index: usize,
    pub debug: bool,
    pub stack: Vec<Value>,
    pub compiler: Compiler,
    pub globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), VMError> {
        let chunks = self.compiler.compile(source);

        if let Ok(parsed_chunks) = chunks {
            self.chunks = parsed_chunks;
        }

        self.run()
    }

    fn run(&mut self) -> Result<(), VMError> {
        dbg!(&self);
        while self.index < self.chunks.len() {
            if self.debug {
                for value in &self.stack {
                    println!("[{}]", value);
                }
                println!("Instruction: {}", &self.chunks[self.index]);
            }
            let op = &self.chunks[self.index];
            dbg!(&self.stack);
            match op {
                OpCode::Constant(value) => self.stack.push(value.clone()),
                OpCode::Return => return Ok(()),
                OpCode::Negate => {
                    let operand = self.stack.pop().unwrap();
                    match operand {
                        Value::Number(num) => self.stack.push(Value::from(-num)),
                        _ => self.runtime_error("Operand must be a number."),
                    }
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.interpret_bin_op(op.clone())
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Not => {
                    let top = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(top.is_falsey()));
                }
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a == b));
                }
                OpCode::Greater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a > b));
                }
                OpCode::Less => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a < b));
                }
                OpCode::Print => {
                    let top = self.stack.pop().unwrap();
                    println!("{}", top);
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::DefineGlobal(name) => {
                    let top = self.stack.pop().unwrap();
                    self.globals.insert(name.to_string(), top);
                }
                OpCode::GetGlobal(name) => match self.globals.get(name) {
                    Some(value) => self.stack.push(value.clone()),
                    None => {
                        self.runtime_error(&format!("Undefined variable '{}'", name));
                        return Err(VMError::Runtime);
                    }
                },
                OpCode::SetGlobal(name) => {
                    if self.globals.contains_key(name) {
                        let val = self.stack.last().unwrap();
                        self.globals.insert(name.to_string(), val.clone());
                    } else {
                        self.runtime_error(&format!("Undefined variable '{}'", name));
                        return Err(VMError::Runtime);
                    }
                }
            }
            dbg!(&self.stack);
            dbg!(&self.globals);
            self.index += 1;
        }
        Ok(())
    }

    fn runtime_error(&self, message: &str) {
        dbg!(message);
    }

    fn interpret_bin_op(&mut self, op: OpCode) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.stack.push(Value::from(match op {
                OpCode::Add => a + b,
                OpCode::Subtract => a - b,
                OpCode::Multiply => a * b,
                OpCode::Divide => a / b,
                _ => unreachable!(),
            })),
            (Value::Obj(Obj::String(a)), Value::Obj(Obj::String(b))) => {
                let mut new_str = a;
                new_str.push_str(&b);
                self.stack.push(Value::Obj(Obj::String(new_str)));
            }
            _ => self.runtime_error("Operands must be two numbers or two strings."),
        }
    }
}
