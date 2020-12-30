mod errors;
mod scanner;

use scanner::Scanner;

pub fn run(source: String) -> Result<(), String> {
    let mut s = Scanner{ source: &source, pos: 0 };
    let tokens = s.scan_tokens();
    println!("running {}", source);
    println!("tokens {:?}", tokens);

    Ok(())

}