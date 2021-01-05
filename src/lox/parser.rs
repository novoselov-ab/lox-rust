use super::ast::*;
use super::errors;
use super::tokens::*;
use errors::CompileErrorType::*;
use errors::{CompileError, CompileErrorType};

use TokenType::*;

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token<'a>>,
    pub statements: Vec<Stmt<'a>>,
    pub current: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'a> {
    Expression(Box<Expr<'a>>),
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            statements: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(self: &mut Self) -> Result<&Vec<Stmt<'a>>, CompileError> {
        while !self.is_at_end() {
            let e = self.expression()?;
            self.statements.push(Stmt::Expression(Box::new(e)));
        }

        Ok(&self.statements)
    }

    fn expression(&mut self) -> Result<Expr<'a>, CompileError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, CompileError> {
        let mut exp = self.comparison()?;

        while let Some(op) = self.match_tokens(&[BangEqual, EqualEqual]) {
            exp = Expr::Binary(Box::new(exp), op, Box::new(self.comparison()?));
        }

        return Ok(exp);
    }

    fn comparison(&mut self) -> Result<Expr<'a>, CompileError> {
        let mut exp = self.term()?;
        while let Some(op) = self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            exp = Expr::Binary(Box::new(exp), op, Box::new(self.term()?));
        }

        return Ok(exp);
    }

    fn term(&mut self) -> Result<Expr<'a>, CompileError> {
        let mut exp = self.factor()?;
        while let Some(op) = self.match_tokens(&[Minus, Plus]) {
            exp = Expr::Binary(Box::new(exp), op, Box::new(self.factor()?));
        }

        return Ok(exp);
    }

    fn factor(&mut self) -> Result<Expr<'a>, CompileError> {
        let mut exp = self.unary()?;
        while let Some(op) = self.match_tokens(&[Slash, Star]) {
            exp = Expr::Binary(Box::new(exp), op, Box::new(self.unary()?));
        }

        return Ok(exp);
    }

    fn unary(&mut self) -> Result<Expr<'a>, CompileError> {
        if let Some(op) = self.match_tokens(&[Bang, Minus]) {
            return Ok(Expr::Unary(op, Box::new(self.unary()?)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr<'a>, CompileError> {
        if let Some(t) = self.match_tokens(&[False, True, Nil, Number, Str]) {
            return Ok(Expr::Literal(t));
        }

        if let Some(_) = self.match_tokens(&[LeftParen]) {
            let e = self.expression()?;
            self.consume(&RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(e)));
        }

        self.build_error(InvalidSyntax, "Uexpected expression")
    }

    fn consume(&mut self, ttype: &TokenType, error_msg: &str) -> Result<&Token<'a>, CompileError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }

        self.build_error(InvalidSyntax, error_msg)
    }

    fn build_error<T>(
        self: &Self,
        error_type: CompileErrorType,
        msg: &str,
    ) -> Result<T, CompileError> {
        Err(CompileError {
            err: error_type,
            line: self.previous().line,
            text: if msg.is_empty() {
                None
            } else {
                Some(msg.to_string())
            },
        })
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == *ttype
        }
    }

    fn match_tokens(&mut self, ttypes: &[TokenType]) -> Option<Token<'a>> {
        for ttype in ttypes {
            if self.check(&ttype) {
                return Some(self.advance().clone());
            }
        }
        None
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == Eof
    }
}

#[cfg(test)]
mod tests {
    use super::super::astprinter;
    use super::*;

    use super::super::Scanner;

    fn test_valid_expr(expr: &str) {
        let mut s = Scanner::new(expr);
        let tokens = s.scan_tokens().unwrap();
        let mut p = Parser::new(&tokens);
        println!("tokens: {:?}", tokens);
        let stmts = p.parse();
        let s = &stmts.unwrap()[0];
        match s {
            Stmt::Expression(e) => {
                assert_eq!(astprinter::dump_ast(e), expr);
            }
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn basic() {
        test_valid_expr("1+(3*2+-10)");
        test_valid_expr("2123-23232/2");
        test_valid_expr("(-1)+(!5)");
        test_valid_expr("false+true");
    }
}
