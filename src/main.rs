mod lox;


use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => loop {
            print!("> ");
            let _ = stdout().flush();

            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("String is incorrect.");

            if s.trim() == "exit" {
                break;
            }

            lox::run(s);
        },
        2 => {
            let file = &args[1];
            let content = fs::read_to_string(file).expect("Can't open script file.");
            lox::run(content)?;
        }
        _ => {
            println!("Usage: lox-rust [script]")
        }
    }

    Ok(())
}
