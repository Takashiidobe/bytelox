use std::{fs::read_to_string, io, process};

use crate::vm::{VMError, VM};

pub struct Interpreter {
    args: Vec<String>,
}

impl Interpreter {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn run(&mut self) {
        match self.args.len() {
            1 => self.repl(),
            2 => self.run_file(self.args[1].to_string()).unwrap(),
            _ => {
                eprintln!("Usage: bytelox [path]");
                process::exit(64);
            }
        }
    }

    fn repl(&mut self) {
        let mut line = String::new();
        let mut vm = VM::new();

        loop {
            print!("> ");

            if let Err(e) = io::stdin().read_line(&mut line) {
                eprintln!("{}", e);
                break;
            }

            let _ = vm.interpret(&line);
        }
    }

    fn run_file(&mut self, path: String) -> Result<(), VMError> {
        let source = read_to_string(path).unwrap();
        let mut vm = VM::new();
        vm.interpret(&source)
    }
}
