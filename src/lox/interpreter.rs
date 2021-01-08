use super::ast::*;
use super::errors;
use super::parser::Stmt;
use super::tokens::*;
use super::value::Value;
use errors::CompileErrorType::*;
use errors::{CompileError, CompileErrorType};

use TokenType::*;

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
            }
            _ => {}
        }

        Ok(())
    }

    fn evaluate(&mut self, node: &Box<Expr>) -> Result<Value, CompileError> {
        match **node {
            Expr::Binary(ref left, ref op, ref right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match op.ttype {
                    TokenType::Plus => match (l, r) {
                        (Value::Number(l), Value::Number(r)) => return Ok(Value::Number(l + r)),
                        (Value::Str(l), Value::Str(r)) => return Ok(Value::Str(l + &r)),
                        _ => {
                            return self.build_error(
                                op.line,
                                EvaluationFailed,
                                "Unsupported types sum",
                            )
                        }
                    },
                    TokenType::EqualEqual => return Ok(Value::Bool(l == r)),
                    TokenType::BangEqual => return Ok(Value::Bool(l != r)),
                    _ => {}
                }
            }
            Expr::Literal(ref t) => {
                return Ok(t.literal.as_ref().unwrap().clone());
            }
            _ => {}
        }

        self.build_error(0, EvaluationFailed, "Code path is not covered yet")
    }

    fn build_error<T>(
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
