use super::ast::*;
use super::tokens::*;
use Expr::*;

fn dump_ast(expr: &Expr) -> String {
    return match expr {
        Expr::Identifier(t) => {
            format!("{}", t.lexeme)
        }
        Expr::Literal(t) => match t.ttype {
            TokenType::Str(ref s) => format!("{}", s),
            TokenType::Number(n) => format!("{}", n),
            TokenType::Identifier => format!("{}", t.lexeme),
            _ => panic!("unexpected token type"),
        },
        Expr::Grouping(e) => {
            format!("({})", dump_ast(e))
        }
        Expr::Unary(t, e) => {
            format!("{}{}", t.lexeme, dump_ast(e))
        }
        Expr::Binary(l, t, r) => {
            format!("{}{}{}", dump_ast(l), t.lexeme, dump_ast(r))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use TokenType::*;

        let e = Unary(
            Token {
                ttype: Bang,
                lexeme: "!",
                line: 1,
            },
            Box::new(Expr::Literal(Token {
                ttype: Number(0.),
                lexeme: "0.",
                line: 1,
            })),
        );
        let s = dump_ast(&e);
        assert_eq!(s, "!0");
    }
}
