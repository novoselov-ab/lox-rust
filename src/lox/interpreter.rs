use super::ast::*;
use super::errors;
use super::parser::Stmt;
use super::tokens::*;
use super::value::Value;
use errors::CompileErrorType::*;
use errors::{CompileError, CompileErrorType};

pub struct Interpreter {}

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn exec(self: &mut Self, stmts: &[Stmt]) -> Result<(), CompileError> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }

        Ok(())
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Expression(expr) => {
                let r = self.evaluate(expr)?;
                println!("{}", r);
            } //_ => {}
        }

        Ok(())
    }

    fn evaluate(&mut self, node: &Box<Expr>) -> Result<Value, CompileError> {
        use Value::*;
        return match **node {
            Expr::Binary(ref left, ref op, ref right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match op.ttype {
                    TokenType::Plus => match (l, r) {
                        (Number(l), Number(r)) => Ok(Number(l + r)),
                        (Str(l), Str(r)) => Ok(Str(l + &r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for +"),
                    },
                    TokenType::Minus => match (l, r) {
                        (Number(l), Number(r)) => Ok(Number(l - r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for -"),
                    },
                    TokenType::Star => match (l, r) {
                        (Number(l), Number(r)) => Ok(Number(l * r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for *"),
                    },
                    TokenType::Slash => match (l, r) {
                        (Number(l), Number(r)) => Ok(Number(l / r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for /"),
                    },
                    TokenType::Greater => match (l, r) {
                        (Number(l), Number(r)) => Ok(Bool(l > r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for >"),
                    },
                    TokenType::GreaterEqual => match (l, r) {
                        (Number(l), Number(r)) => Ok(Bool(l >= r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for >="),
                    },
                    TokenType::Less => match (l, r) {
                        (Number(l), Number(r)) => Ok(Bool(l < r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for <"),
                    },
                    TokenType::LessEqual => match (l, r) {
                        (Number(l), Number(r)) => Ok(Bool(l <= r)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for <="),
                    },
                    TokenType::EqualEqual => Ok(Bool(l == r)),
                    TokenType::BangEqual => Ok(Bool(l != r)),
                    _ => self.error(0, EvaluationFailed, "Code path is not covered yet"),
                }
            }
            Expr::Unary(ref op, ref t) => {
                let v = self.evaluate(t)?;
                match op.ttype {
                    TokenType::Minus => match v {
                        Number(n) => Ok(Number(-n)),
                        _ => self.error(op.line, EvaluationFailed, "Unsupported types for unary -"),
                    },
                    TokenType::Bang => Ok(Bool(!v.is_truthy())),
                    _ => self.error(0, EvaluationFailed, "Wrong unary operator"),
                }
            }

            Expr::Literal(ref t) => Ok(t.literal.as_ref().unwrap().clone()),
            Expr::Grouping(ref g) => self.evaluate(g),
            _ => self.error(0, EvaluationFailed, "Code path is not covered yet"),
        };
    }

    fn error<T>(
        self: &Self,
        line: usize,
        error_type: CompileErrorType,
        msg: &str,
    ) -> Result<T, CompileError> {
        Err(CompileError {
            err: error_type,
            line: line,
            text: if msg.is_empty() {
                None
            } else {
                Some(msg.to_string())
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::astprinter;
    use super::*;

    use super::super::Scanner;
}
