mod errors;
mod scanner;
mod parser;
mod tokens;
mod ast;
mod astprinter;

use scanner::Scanner;

pub fn run(source: String) -> Result<(), String> {
    let mut s = Scanner::new(&source);
    match s.scan_tokens() {
        Ok(tokens) => {
            println!("tokens: {:?}", tokens);
        }
        Err(e) => return Err(e.to_string()),
    }

    Ok(())
}
