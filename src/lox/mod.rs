mod errors;
mod scanner;

use scanner::Scanner;

pub fn run(source: String) -> Result<(), String> {
    let mut s = Scanner::new(source);

    let tokens = s.scan_tokens();
    //println!("running {}", source);
    println!("tokens {:?}", tokens);

    Ok(())
}
