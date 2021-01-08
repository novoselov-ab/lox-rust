mod ast;
mod astprinter;
mod errors;
mod parser;
mod scanner;
mod tokens;
mod interpreter;
mod value;

use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;

pub fn run(source: String) -> Result<(), String> {
    let mut s = Scanner::new(&source);
    match s.scan_tokens() {
        Ok(tokens) => {

            let mut p = Parser::new(&tokens);
            let stmts = p.parse();

            match stmts {
                Ok(stmts) => {
                    let mut interpreter = Interpreter::new();
                    println!("tokens: {:?}", stmts);

                    if let Err(e) = interpreter.exec(stmts) {
                        return Err(e.to_string());
                    }
                }
                Err(e) => return Err(e.to_string()),        
            }
    
        }
        Err(e) => return Err(e.to_string()),
    }

    Ok(())
}
