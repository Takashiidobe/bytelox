use std::{fs::read_to_string, io, process};

use crate::vm::{VMError, VM};

pub struct Interpreter {
    args: Vec<String>,
    vm: VM,
}

impl Interpreter {
    pub fn new(vm: VM, args: Vec<String>) -> Self {
        Self { vm, args }
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

        loop {
            print!("> ");

            match io::stdin().read_line(&mut line) {
                Ok(_) => println!(),
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }

            let _ = self.vm.interpret();
        }
    }

    fn run_file(&mut self, path: String) -> Result<(), VMError> {
        let source = read_to_string(path).unwrap();
        let interpret_result = self.vm.interpret();

        match interpret_result {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
