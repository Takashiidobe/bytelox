/*
use std::{collections::HashMap, sync::LazyLock};

use crate::{
    opcode::OpCode,
    scanner::{Scanner, Token, TokenType, TokenValue},
    value::Value,
    vm::VMError,
};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Compiler {
    source: String,
    parser: Parser,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        let mut parser = Parser::new();
        Self { source, parser }
    }

    pub fn compile(&self) -> Result<Vec<Token>, VMError> {
        let mut scanner = Scanner::new(self.source.chars().collect());
        let mut chunks = vec![];

        advance();
        expression();
        consume(TokenType::Eof, "Expect end of expression.");
        end_compiler();
        if parser.had_error {
            Err(VMError::CompileTime)
        } else {
            Ok(vec![scanner.scan_token()])
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Precedence {
    #[default]
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
    Top,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Assignment,
            2 => Self::Or,
            3 => Self::And,
            4 => Self::Equality,
            5 => Self::Comparison,
            6 => Self::Term,
            7 => Self::Factor,
            8 => Self::Unary,
            9 => Self::Call,
            10 => Self::Primary,
            11 => Self::Top,
            0 | _ => Self::None,
        }
    }
}

impl Into<u8> for Precedence {
    fn into(self) -> u8 {
        match self {
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::Or => 2,
            Precedence::And => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Unary => 8,
            Precedence::Call => 9,
            Precedence::Primary => 10,
            Precedence::Top => 11,
        }
    }
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum PrefixRule {
    Grouping,
    Unary,
    Number,
    #[default]
    None,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum InfixRule {
    Binary,
    #[default]
    None,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ParseRule {
    prefix: PrefixRule,
    infix: InfixRule,
    precedence: Precedence,
}

static RULES: LazyLock<HashMap<TokenType, ParseRule>> = LazyLock::new(|| {
    HashMap::from([
        (
            TokenType::LeftParen,
            ParseRule {
                prefix: PrefixRule::Grouping,
                ..Default::default()
            },
        ),
        (
            TokenType::Minus,
            ParseRule {
                prefix: PrefixRule::Unary,
                infix: InfixRule::Binary,
                precedence: Precedence::Term,
            },
        ),
        (
            TokenType::Plus,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Term,
                ..Default::default()
            },
        ),
        (
            TokenType::Slash,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Factor,
                ..Default::default()
            },
        ),
        (
            TokenType::Star,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Factor,
                ..Default::default()
            },
        ),
        (
            TokenType::Number,
            ParseRule {
                prefix: PrefixRule::Number,
                ..Default::default()
            },
        ),
    ])
});

fn get_rule(token_type: &TokenType) -> ParseRule {
    RULES.get(token_type).cloned().unwrap_or_default()
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
    pub had_error: bool,
    pub panic_mode: bool,
    pub source: String,
}

impl Parser {
    pub fn error(&mut self, message: &str) {
        self.error_at(message);
    }

    pub fn consume(&mut self, token: TokenType, message: &str) {
        if self.tokens[self.index].r#type == token {
            self.advance();
        } else {
            self.error_at(message);
        }
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        if !self.had_error {
            dbg!(&self.tokens[self.index]);
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        self.ops.push(opcode);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn number(&mut self) {
        let Token { value, .. } = &self.tokens[self.index - 1];

        match value {
            TokenValue::Number(num) => self.emit_constant(Value::from(*num)),
            _ => unreachable!(),
        }
    }

    fn emit_constant(&mut self, value: Value) {
        self.emit_byte(OpCode::Constant(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.")
    }

    fn unary(&mut self) {
        self.parse_precedence(Precedence::Unary);

        let operator_type = self.tokens[self.index - 1].r#type.clone();

        self.expression();

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator_type = &self.tokens[self.index - 1].r#type;
        let rule = get_rule(operator_type);

        self.parse_precedence(Precedence::from(
            std::convert::Into::<u8>::into(rule.precedence) + 1u8,
        ));

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => unreachable!(),
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&self, precedence: Precedence) {
        self.advance();
        let prefix_rule = get_rule(self.previous.r#type).prefix;

        prefix_rule();

        while precedence <= get_rule(self.current.r#type).precedence {
            self.advance();
            let infix_rule = get_rule(self.previous.r#type).infix;
            self.infix_rule();
        }
    }

    fn error_at(&mut self, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        let token = &self.tokens[self.index];

        eprint!("[line {}] Error", token.line);

        match token.r#type {
            TokenType::Eof => {
                eprint!(" at end");
            }
            TokenType::Error => {}
            _ => {
                eprint!(" at {} to {}", token.start, token.start + token.length);
            }
        }
        eprintln!(": {}", message);
        self.had_error = true;
    }
}

*/
