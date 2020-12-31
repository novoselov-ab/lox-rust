use std::{error, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum CompileErrorType {
    UnexpectedChar(char),
    UnterminatedString,
}

#[derive(Debug, PartialEq)]
pub struct CompileError {
    pub err: CompileErrorType,
    pub line: usize,
    pub text: Option<String>,
}

impl error::Error for CompileError {}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] Error", self.line)?;
        if let Some(t) = &self.text {
            write!(f, " at '{}'", t)?;
        }
        write!(f, ": {:?}", self.err)
    }
}

//static mut had_error: bool = false;

fn report(line: usize, source: &str, message: &str) {
    println!("[line {}] Error{}: {}", line, source, message);
    //had_error = true;
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}
