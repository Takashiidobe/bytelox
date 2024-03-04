use bytelox::{interpreter::Interpreter, opcode::OpCode, vm::VM};
use std::env;

fn main() {
    let mut vm = VM::interpret(&mut self, "1 + 2");
    let mut interpreter = Interpreter::new(vm, env::args().collect());

    interpreter.run();
}
