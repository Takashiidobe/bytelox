use crate::scanner::{Scanner, Token};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Compiler {
    source: String,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn compile(&self) -> Token {
        let mut scanner = Scanner::new(self.source.chars().collect());
        scanner.scan_token()
    }
}
