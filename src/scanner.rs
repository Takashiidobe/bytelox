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
        if self.is_at_end() {
            return Token::Eof {
                start: self.current,
                line: self.line,
            };
        }
        let c = self.advance();
        match c {
            '(' => Token::LeftParen {
                start: self.current,
                line: self.line,
            },
            ')' => Token::RightParen {
                start: self.current,
                line: self.line,
            },
            '{' => Token::LeftBrace {
                start: self.current,
                line: self.line,
            },
            '}' => Token::RightBrace {
                start: self.current,
                line: self.line,
            },
            ';' => Token::Semicolon {
                start: self.current,
                line: self.line,
            },
            '.' => Token::Dot {
                start: self.current,
                line: self.line,
            },
            ',' => Token::Comma {
                start: self.current,
                line: self.line,
            },
            '-' => Token::Minus {
                start: self.current,
                line: self.line,
            },
            '+' => Token::Plus {
                start: self.current,
                line: self.line,
            },
            '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Token::Comment {
                        start: self.current,
                        line: self.line,
                    }
                } else {
                    Token::Slash {
                        start: self.current,
                        line: self.line,
                    }
                }
            }
            '*' => Token::Star {
                start: self.current,
                line: self.line,
            },
            '!' => {
                if self.r#match('=') {
                    Token::BangEqual {
                        start: self.current,
                        line: self.line,
                    }
                } else {
                    Token::Bang {
                        start: self.current,
                        line: self.line,
                    }
                }
            }
            '<' => {
                if self.r#match('=') {
                    Token::LessEqual {
                        start: self.current,
                        line: self.line,
                    }
                } else {
                    Token::Less {
                        start: self.current,
                        line: self.line,
                    }
                }
            }
            '>' => {
                if self.r#match('=') {
                    Token::GreaterEqual {
                        start: self.current,
                        line: self.line,
                    }
                } else {
                    Token::Greater {
                        start: self.current,
                        line: self.line,
                    }
                }
            }
            '=' => {
                if self.r#match('=') {
                    Token::EqualEqual {
                        start: self.current,
                        line: self.line,
                    }
                } else {
                    Token::Equal {
                        start: self.current,
                        line: self.line,
                    }
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(c),
            _ => Token::Error {
                message: format!("Unknown Token {}", c),
                start: self.current,
                line: self.line,
            },
        }
    }

    fn identifier(&mut self, c: char) -> Token {
        let potential_matches = self.identifiers.entry(c).or_default().clone();
        for keyword in potential_matches {
            if self.check_keyword(&keyword) {
                return Token::Identifier {
                    value: keyword.to_string(),
                    start: self.current,
                    line: self.line,
                };
            }
        }
        unreachable!()
    }

    fn check_keyword(&self, keyword: &str) -> bool {
        if self.current + keyword.len() >= self.source.len() {
            return false;
        }

        let mut temp_index = self.current;

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

        Token::Number {
            value: value.parse().unwrap(),
            start: self.current,
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        let mut value = String::new();
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            value.push(self.advance());
        }

        if self.is_at_end() {
            return Token::Error {
                message: "Unterminated String".to_string(),
                start: self.current,
                line: self.line,
            };
        }

        self.advance();

        Token::String {
            value,
            start: self.current,
            line: self.line,
        }
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn skip_whitespace(&mut self) {
        let c = self.peek();
        while c == ' ' || c == '\r' || c == '\t' || c == '\n' {
            self.advance();
            if c == '\n' {
                self.line += 1;
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

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn advance(&mut self) -> char {
        let curr_char = self.peek();
        self.current += 1;
        curr_char
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    // Single Character Tokens
    LeftParen {
        start: usize,
        line: usize,
    },
    RightParen {
        start: usize,
        line: usize,
    },
    LeftBrace {
        start: usize,
        line: usize,
    },
    RightBrace {
        start: usize,
        line: usize,
    },
    Comma {
        start: usize,
        line: usize,
    },
    Dot {
        start: usize,
        line: usize,
    },
    Minus {
        start: usize,
        line: usize,
    },
    Plus {
        start: usize,
        line: usize,
    },
    Semicolon {
        start: usize,
        line: usize,
    },
    Slash {
        start: usize,
        line: usize,
    },
    Star {
        start: usize,
        line: usize,
    },
    // One or two character tokens
    Bang {
        start: usize,
        line: usize,
    },
    BangEqual {
        start: usize,
        line: usize,
    },
    Equal {
        start: usize,
        line: usize,
    },
    EqualEqual {
        start: usize,
        line: usize,
    },
    Greater {
        start: usize,
        line: usize,
    },
    GreaterEqual {
        start: usize,
        line: usize,
    },
    Less {
        start: usize,
        line: usize,
    },
    LessEqual {
        start: usize,
        line: usize,
    },
    // Literals
    Identifier {
        value: String,
        start: usize,
        line: usize,
    },
    String {
        value: String,
        start: usize,
        line: usize,
    },
    Number {
        value: f64,
        start: usize,
        line: usize,
    },
    // Keywords
    And {
        start: usize,
        line: usize,
    },
    Class {
        start: usize,
        line: usize,
    },
    Else {
        start: usize,
        line: usize,
    },
    False {
        start: usize,
        line: usize,
    },
    For {
        start: usize,
        line: usize,
    },
    Fun {
        start: usize,
        line: usize,
    },
    If {
        start: usize,
        line: usize,
    },
    Nil {
        start: usize,
        line: usize,
    },
    Or {
        start: usize,
        line: usize,
    },
    Print {
        start: usize,
        line: usize,
    },
    Return {
        start: usize,
        line: usize,
    },
    Super {
        start: usize,
        line: usize,
    },
    This {
        start: usize,
        line: usize,
    },
    True {
        start: usize,
        line: usize,
    },
    Var {
        start: usize,
        line: usize,
    },
    While {
        start: usize,
        line: usize,
    },
    Error {
        message: String,
        start: usize,
        line: usize,
    },
    Eof {
        start: usize,
        line: usize,
    },
    Comment {
        start: usize,
        line: usize,
    },
}
