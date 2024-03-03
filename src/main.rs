use bytelox::{opcode::OpCode, vm::VM};

fn main() {
    let instructions = vec![
        OpCode::from(1.2),
        OpCode::from(3.4),
        OpCode::Add,
        OpCode::from(5.6),
        OpCode::Divide,
        OpCode::Negate,
        OpCode::Return,
    ];
    let mut vm = VM::init(instructions.clone());
    vm.debug = true;

    let _ = vm.interpret();
}
