use super::tokens::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Identifier(Token<'a>),
    Literal(Token<'a>),
    Grouping(Box<Expr<'a>>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    // Assignment(Token<'a>, Box<Expr<'a>>),
    // Call(Box<Expr<'a>>, Token<'a>, Vec<Expr<'a>>),
}
