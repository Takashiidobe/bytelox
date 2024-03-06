/*
use std::fmt;


use crate::{compiler::Compiler, opcode::OpCode, value::Value};

#[derive(Debug, Clone)]
pub enum VMError {
    CompileTime,
    Runtime,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::CompileTime => write!(f, "compiler error"),
            VMError::Runtime => write!(f, "runtime error"),
        }
    }
}

pub struct VM {
    chunks: Vec<OpCode>,
    index: usize,
    pub debug: bool,
    stack: Vec<Value>,
    compiler: Compiler,
}

impl VM {
    pub fn new(source: &str) -> Self {
        Self {
            chunks: vec![],
            index: 0,
            debug: false,
            stack: vec![],
            compiler: Compiler::new(source.to_string()),
        }
    }

    pub fn interpret(&mut self) -> Result<(), VMError> {
        let chunks = self.compiler.compile();
        if let Ok(parsed_chunks) = chunks {
            self.chunks = parsed_chunks;
        }

        self.run()
    }

    fn run(&mut self) -> Result<(), VMError> {
        while self.index < self.chunks.len() {
            if self.debug {
                for value in &self.stack {
                    println!("[{}]", value);
                }
                println!("Instruction: {}", &self.chunks[self.index]);
            }
            let op = &self.chunks[self.index];
            match op {
                OpCode::Constant(value) => self.stack.push(value.clone()),
                OpCode::Return => println!("{}\n", self.stack.pop().unwrap()),
                OpCode::Negate => {
                    let operand = self.stack.pop().unwrap();
                    match operand {
                        Value::Number(num) => self.stack.push(Value::from(-num)),
                    }
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.interpret_bin_op(op.clone())
                }
            }
            self.index += 1;
        }
        Ok(())
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
        }
    }
}
*/
