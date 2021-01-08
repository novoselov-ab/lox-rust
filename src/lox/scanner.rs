use super::errors;
use super::tokens::*;
use super::value::*;
use errors::CompileErrorType::*;
use errors::{CompileError, CompileErrorType};
use std::collections::HashMap;

use TokenType::*;

pub struct Scanner<'a> {
    pub source: &'a str,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub tokens: Vec<Token<'a>>,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("and", And);
        keywords.insert("class", Class);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("for", For);
        keywords.insert("fun", Fun);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("or", Or);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("super", Super);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("var", Var);
        keywords.insert("while", While);

        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: keywords,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<&Vec<Token>, CompileError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            ttype: Eof,
            lexeme: "",
            literal: None,
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
            '0'..='9' => {
                self.scan_number();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.scan_identifier();
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

    fn peek_next(self: &mut Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
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

        self.add_token_with_literal(
            Str,
            Some(Value::Str(
                self.source[self.start + 1..self.current - 1].to_string(),
            )),
        );

        Ok(())
    }

    fn scan_number(self: &mut Self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token_with_literal(
            Number,
            Some(Value::Number(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            )),
        );
    }

    fn scan_identifier(self: &mut Self) {
        loop {
            let c = self.peek();
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let value = &self.source[self.start..self.current];

        let ttype = self.keywords.get(value).unwrap_or(&Identifier).clone();
        self.add_token(ttype);
    }

    fn add_token(self: &mut Self, ttype: TokenType) {
        self.add_token_with_literal(ttype, None)
    }

    fn add_token_with_literal(self: &mut Self, ttype: TokenType, literal: Option<Value>) {
        self.tokens.push(Token {
            ttype: ttype,
            lexeme: &self.source[self.start..self.current],
            literal,
            line: self.line,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan_types(msg: &str) -> Vec<TokenType> {
        let mut s = Scanner::new(msg);
        let tokens = s.scan_tokens().unwrap();
        tokens.iter().map(|t| t.ttype.clone()).collect()
    }

    fn scan_error(msg: &str) -> CompileError {
        let mut s = Scanner::new(msg);
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
        assert_eq!(scan_types("(123450.6789"), vec![LeftParen, Number, Eof]);
        assert_eq!(scan_types("else or lol"), vec![Else, Or, Identifier, Eof]);
    }

    #[test]
    fn unexpected_char_error() {
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
