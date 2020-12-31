use errors::CompileErrorType::*;
use errors::{CompileError, CompileErrorType};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    Str,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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

    Eof,
}

use TokenType::*;

use super::errors;

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<&Vec<Token>, CompileError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            ttype: Eof,
            lexeme: "".to_string(),
            line: self.line,
        });

        Ok(&self.tokens)
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self: &mut Self) -> Result<(), CompileError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let next_eq = self.advance_if_match('=');
                self.add_token(if next_eq { BangEqual } else { Bang });
            }
            '=' => {
                let next_eq = self.advance_if_match('=');
                self.add_token(if next_eq { EqualEqual } else { Bang });
            }
            '<' => {
                let next_eq = self.advance_if_match('=');
                self.add_token(if next_eq { LessEqual } else { Less });
            }
            '>' => {
                let next_eq = self.advance_if_match('=');
                self.add_token(if next_eq { GreaterEqual } else { Greater });
            }
            '/' => {
                let next_slash = self.advance_if_match('/');
                if next_slash {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.scan_string()?;
            }
            _ => {
                return self.build_error(UnexpectedChar(c), "");
            }
        }

        Ok(())
    }

    fn build_error(
        self: &Self,
        error_type: CompileErrorType,
        msg: &str,
    ) -> Result<(), CompileError> {
        Err(CompileError {
            err: error_type,
            line: self.line,
            text: if msg.is_empty() {
                None
            } else {
                Some(msg.to_string())
            },
        })
    }

    fn advance(self: &mut Self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn advance_if_match(self: &mut Self, c: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(self: &mut Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn scan_string(self: &mut Self) -> Result<(), CompileError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.build_error(UnterminatedString, "");
        }

        // close "
        self.advance();

        self.tokens.push(Token {
            ttype: Str,
            lexeme: self.source[self.start + 1..self.current - 1].to_string(),
            line: self.line,
        });

        Ok(())
    }

    fn add_token(self: &mut Self, ttype: TokenType) {
        self.tokens.push(Token {
            ttype: ttype,
            lexeme: self.source[self.start..self.current].to_string(),
            line: self.line,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan_types(msg: &str) -> Vec<TokenType> {
        let mut s = Scanner::new(msg.to_string());
        let tokens = s.scan_tokens().unwrap();
        tokens.iter().map(|t| t.ttype).collect()
    }

    fn scan_error(msg: &str) -> CompileError {
        let mut s = Scanner::new(msg.to_string());
        s.scan_tokens().err().unwrap()
    }

    #[test]
    fn basic() {
        assert_eq!(
            scan_types("()!=!"),
            vec![LeftParen, RightParen, BangEqual, Bang, Eof]
        );
        assert_eq!(scan_types("(//==\n=="), vec![LeftParen, EqualEqual, Eof]);
        assert_eq!(
            scan_types("(\"!===\")"),
            vec![LeftParen, Str, RightParen, Eof]
        );
    }

    #[test]
    fn unexpected_char_error() {
        let err = scan_error("((~");
        assert_eq!(
            scan_error("\n((\n(~"),
            CompileError {
                err: UnexpectedChar('~'),
                line: 3,
                text: None
            }
        );
        assert_eq!(
            scan_error("(\"===adsdas\n"),
            CompileError {
                err: UnterminatedString,
                line: 2,
                text: None
            }
        );
    }
}
