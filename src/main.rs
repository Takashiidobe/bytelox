use bytelox::interpreter::Interpreter;
use std::env;

fn main() {
    let args = env::args().collect();
    let mut interpreter = Interpreter::new(args);

    interpreter.run();
}
