use std::{collections::HashMap, ops::Add};

use lazy_static::lazy_static;

use crate::{
    opcode::OpCode,
    scanner::{Scanner, Token, TokenType, TokenValue},
    value::{Obj, Value},
    vm::VMError,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Compiler {
    parser: Parser,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(&mut self, source: &str) -> Result<Vec<OpCode>, VMError> {
        self.parser.scanner.input(source);
        self.parser.advance();
        self.parser.expression();
        self.parser
            .consume(&TokenType::Eof, "Expect end of expression.");
        self.parser.end_compiler();

        if self.parser.had_error {
            Err(VMError::CompileTime)
        } else {
            Ok(self.parser.ops.clone())
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
            0 => Self::None,
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
            _ => Self::None,
        }
    }
}

impl Add<u8> for Precedence {
    type Output = Precedence;

    fn add(self, other: u8) -> Precedence {
        Precedence::from(self.clone() as u8 + other)
    }
}

impl From<Precedence> for u8 {
    fn from(val: Precedence) -> Self {
        match val {
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
    #[default]
    None,
    Grouping,
    Unary,
    Number,
    Literal,
    String,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum InfixRule {
    #[default]
    None,
    Binary,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ParseRule {
    prefix: PrefixRule,
    infix: InfixRule,
    precedence: Precedence,
}

lazy_static! {
    static ref RULES: HashMap<TokenType, ParseRule> = HashMap::from([
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
        (
            TokenType::False,
            ParseRule {
                prefix: PrefixRule::Literal,
                ..Default::default()
            },
        ),
        (
            TokenType::True,
            ParseRule {
                prefix: PrefixRule::Literal,
                ..Default::default()
            },
        ),
        (
            TokenType::Nil,
            ParseRule {
                prefix: PrefixRule::Literal,
                ..Default::default()
            },
        ),
        (
            TokenType::Bang,
            ParseRule {
                prefix: PrefixRule::Unary,
                ..Default::default()
            },
        ),
        (
            TokenType::BangEqual,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Equality,
                ..Default::default()
            },
        ),
        (
            TokenType::EqualEqual,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Equality,
                ..Default::default()
            },
        ),
        (
            TokenType::Greater,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Comparison,
                ..Default::default()
            },
        ),
        (
            TokenType::GreaterEqual,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Comparison,
                ..Default::default()
            },
        ),
        (
            TokenType::Less,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Comparison,
                ..Default::default()
            },
        ),
        (
            TokenType::LessEqual,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Comparison,
                ..Default::default()
            },
        ),
        (
            TokenType::String,
            ParseRule {
                prefix: PrefixRule::String,
                ..Default::default()
            },
        ),
    ]);
}

fn get_rule(token_type: &TokenType) -> ParseRule {
    RULES.get(token_type).cloned().unwrap_or_default()
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Parser {
    pub scanner: Scanner,
    pub previous: Option<Token>,
    pub current: Option<Token>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub ops: Vec<OpCode>,
    pub debug: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn consume(&mut self, token_type: &TokenType, message: &str) {
        if let Some(current) = &self.current {
            if current.r#type == *token_type {
                self.advance();
                return;
            }
        }
        self.error(message);
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = Some(self.scanner.scan_token());
            if let Some(current) = &self.current {
                if current.r#type != TokenType::Error {
                    break;
                }
            }
            self.error("found error token");
        }
    }

    fn error(&mut self, message: &str) {
        if let Some(token) = &self.current {
            self.error_at(&token.clone(), message);
        }
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

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

    fn end_compiler(&mut self) {
        self.emit_return();
        if !self.had_error && self.debug {
            dbg!(&self.current);
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        self.ops.push(opcode);
    }

    fn emit_bytes(&mut self, bytes: &[OpCode]) {
        for byte in bytes {
            self.emit_byte(byte.clone());
        }
    }

    fn emit_constant(&mut self, value: Value) {
        self.emit_byte(OpCode::Constant(value));
    }

    fn number(&mut self) {
        if let Some(Token { value, .. }) = &self.previous {
            if let Some(TokenValue::Number(num)) = value {
                self.emit_constant(Value::from(*num))
            }
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(&TokenType::RightParen, "Expect ')' after expression.")
    }

    fn unary(&mut self) {
        let operator_type = self.previous.as_ref().unwrap().r#type.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            TokenType::Bang => self.emit_byte(OpCode::Not),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator_type = self.previous.as_ref().unwrap().r#type.clone();

        let rule_precedence = get_rule(&operator_type).precedence + 1;

        self.parse_precedence(rule_precedence);

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            TokenType::BangEqual => self.emit_bytes(&[OpCode::Equal, OpCode::Not]),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal),
            TokenType::Greater => self.emit_byte(OpCode::Greater),
            TokenType::GreaterEqual => self.emit_bytes(&[OpCode::Less, OpCode::Not]),
            TokenType::Less => self.emit_byte(OpCode::Less),
            TokenType::LessEqual => self.emit_bytes(&[OpCode::Greater, OpCode::Not]),
            _ => unreachable!(),
        }
    }

    fn literal(&mut self) {
        let operator_type = self.previous.as_ref().unwrap().r#type.clone();
        match operator_type {
            TokenType::False => self.emit_byte(OpCode::False),
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            TokenType::True => self.emit_byte(OpCode::True),
            _ => unreachable!(),
        }
    }

    fn string(&mut self) {
        if let Some(ref x) = self.previous {
            if TokenType::String == x.r#type {
                let value = x.value.as_ref().unwrap();
                if let TokenValue::String(s) = value {
                    self.emit_constant(Value::Obj(Obj::String(s.to_string())))
                }
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = get_rule(&self.previous.as_ref().unwrap().r#type).prefix;

        match prefix_rule {
            PrefixRule::Grouping => self.grouping(),
            PrefixRule::Unary => self.unary(),
            PrefixRule::Number => self.number(),
            PrefixRule::Literal => self.literal(),
            PrefixRule::String => self.string(),
            PrefixRule::None => self.error("Expected expression"),
        }

        while precedence <= get_rule(&self.current.as_ref().unwrap().r#type).precedence {
            self.advance();
            let infix_rule = get_rule(&self.previous.as_ref().unwrap().r#type).infix;
            if let InfixRule::Binary = infix_rule {
                self.binary()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_compiler(source: &str) -> Result<Vec<OpCode>, VMError> {
        let mut compiler = Compiler::new();
        compiler.compile(source)
    }

    macro_rules! test_compiler {
        ($test_name:ident, $source:expr) => {
            #[test]
            fn $test_name() {
                let tokens = test_compiler($source).unwrap();

                insta::assert_yaml_snapshot!(tokens);
            }
        };
    }

    test_compiler!(unary_minus, "-10.23");
    test_compiler!(math, "10.23 - 30 * -20");
    test_compiler!(precedence, "10 + 20 * 30");
    test_compiler!(grouping, "(10 + 20) * 30");
    test_compiler!(gte_false, "10 >= 20");
    test_compiler!(gte_true, "20 >= 10");
    test_compiler!(gte_same, "10 >= 10");
    test_compiler!(lte_false, "20 <= 10");
    test_compiler!(lte_true, "20 <= 10");
    test_compiler!(lte_same, "10 <= 10");
    test_compiler!(ee_true, "10 == 10");
    test_compiler!(ee_false, "10 == 20");
    test_compiler!(ne_true, "10 != 10");
    test_compiler!(ne_false, "10 != 20");
    test_compiler!(string_concat, "\"hello\" + \"world\" + \"from\" + \"rust\"");
}
