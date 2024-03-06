use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
    identifiers: HashMap<char, Vec<String>>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            current: 0,
            identifiers: HashMap::from([
                ('a', vec!["and".to_string()]),
                ('c', vec!["class".to_string()]),
                ('e', vec!["else".to_string()]),
                (
                    'f',
                    vec!["for".to_string(), "fun".to_string(), "false".to_string()],
                ),
                ('i', vec!["if".to_string()]),
                ('n', vec!["nil".to_string()]),
                ('o', vec!["or".to_string()]),
                ('p', vec!["print".to_string()]),
                ('r', vec!["return".to_string()]),
                ('s', vec!["super".to_string()]),
                ('t', vec!["this".to_string(), "true".to_string()]),
                ('v', vec!["var".to_string()]),
                ('w', vec!["while".to_string()]),
            ]),
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.is_at_end() {
            return Token {
                value: None,
                r#type: TokenType::Eof,
                length: 1,
                start: self.current,
                line: self.line,
            };
        }
        let c = self.advance();
        match c {
            '(' | ')' | '{' | '}' | ';' | '.' | ',' | '-' | '+' | '*' => Token {
                value: None,
                r#type: TokenType::from(c),
                length: 1,
                start: self.current,
                line: self.line,
            },
            '/' => {
                if self.peek_next() == '/' {
                    let mut length = 1;
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                        length += 1;
                    }
                    Token {
                        value: None,
                        r#type: TokenType::Comment,
                        start: self.current,
                        line: self.line,
                        length,
                    }
                } else {
                    Token {
                        value: None,
                        r#type: TokenType::Slash,
                        length: 1,
                        start: self.current,
                        line: self.line,
                    }
                }
            }
            '!' | '=' | '<' | '>' => self.relational(c),
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(c),
            _ => Token {
                value: Some(TokenValue::Error(format!("Unknown Token {}", c))),
                r#type: TokenType::Error,
                start: self.current,
                length: 1,
                line: self.line,
            },
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let curr = self.scan_token();
            tokens.push(curr.clone());
            match curr.r#type {
                TokenType::Error => panic!("Error"),
                TokenType::Eof => break,
                _ => {}
            }
        }
        tokens
    }

    fn relational(&mut self, c: char) -> Token {
        let rel_eq = format!("{}=", c);

        if self.r#match('=') {
            Token {
                value: None,
                r#type: TokenType::from(rel_eq.as_str()),
                length: 2,
                start: self.current,
                line: self.line,
            }
        } else {
            Token {
                value: None,
                r#type: TokenType::from(c),
                length: 1,
                start: self.current,
                line: self.line,
            }
        }
    }

    fn identifier(&mut self, c: char) -> Token {
        let potential_matches = self.identifiers.entry(c).or_default().clone();
        let start = self.current.saturating_sub(1);
        for keyword in potential_matches {
            if self.check_keyword(&keyword) {
                self.current += keyword.len();
                return Token {
                    value: None,
                    r#type: TokenType::from(keyword.as_str()),
                    length: keyword.len(),
                    start,
                    line: self.line,
                };
            }
        }

        let mut identifier = String::new();
        identifier.push(self.prev());

        loop {
            let c = self.advance();
            if c.is_ascii_alphanumeric() {
                identifier.push(c);
            } else {
                break;
            }
        }

        let length = identifier.len();

        Token {
            value: Some(TokenValue::Identifier(identifier)),
            r#type: TokenType::Identifier,
            length,
            start,
            line: self.line,
        }
    }

    fn check_keyword(&self, keyword: &str) -> bool {
        if self.current + keyword.len() >= self.source.len() {
            return false;
        }

        let mut temp_index = self.current.saturating_sub(1);

        for c in keyword.chars() {
            if self.source[temp_index] != c {
                return false;
            }
            temp_index += 1;
        }

        true
    }

    fn number(&mut self) -> Token {
        let mut value = String::new();
        let start = self.current.saturating_sub(1);

        value.push(self.prev());

        while self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            value.push(self.advance());

            while self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        let length = value.len();
        let token_type = TokenType::Number;

        Token {
            value: Some(TokenValue::Number(value.parse::<f64>().unwrap())),
            r#type: token_type,
            length,
            start,
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        let mut value = String::new();
        let start = self.current - 1;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            value.push(self.advance());
        }

        if self.is_at_end() {
            let error = "Unterminated string".to_string();
            let length = error.len();
            return Token {
                value: Some(TokenValue::Error(error)),
                r#type: TokenType::Error,
                start,
                length,
                line: self.line,
            };
        }

        self.advance();

        let length = value.len() + 2;

        Token {
            value: Some(TokenValue::String(value)),
            r#type: TokenType::String,
            start,
            length,
            line: self.line,
        }
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                }
                _ => break,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn prev(&self) -> char {
        if self.current == 0 {
            '\0'
        } else {
            self.source[self.current - 1]
        }
    }

    fn peek(&self) -> char {
        if self.current < self.source.len() {
            self.source[self.current]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) -> char {
        let curr_char = self.peek();
        self.current += 1;
        curr_char
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum TokenType {
    // One character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    Comment,
    #[default]
    Eof,
}

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        match value {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            '!' => TokenType::Bang,
            '=' => TokenType::Equal,
            '>' => TokenType::Greater,
            '<' => TokenType::Less,
            _ => panic!("Cannot parse from char: {}", value),
        }
    }
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            "." => TokenType::Dot,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            "/" => TokenType::Slash,
            "*" => TokenType::Star,
            "!" => TokenType::Bang,
            "!=" => TokenType::BangEqual,
            "=" => TokenType::Equal,
            "==" => TokenType::EqualEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => panic!("Couldn't parse from str: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TokenValue {
    Identifier(String),
    String(String),
    Error(String),
    Number(f64),
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Token {
    pub value: Option<TokenValue>,
    pub r#type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_scanner(source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens()
    }

    macro_rules! test_scanner {
        ($test_name:ident, $source:expr) => {
            #[test]
            fn $test_name() {
                let source = $source;
                let tokens = test_scanner(source);

                insta::assert_yaml_snapshot!(tokens);
            }
        };
    }

    test_scanner!(add_numbers, "10.23    + 20.6");
    test_scanner!(var_decl, "var x = 10;");
    test_scanner!(string, "\"hello\"");
    test_scanner!(relational, "10 <= 20");
    test_scanner!(keywords, "for while print return or nil");
    test_scanner!(multiline, "10\n20\n30");
}
