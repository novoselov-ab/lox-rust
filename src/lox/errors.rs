

//static mut had_error: bool = false;

fn report(line: usize, source: &str, message: &str)
{
    println!("[line {}] Error{}: {}", line, source, message);    
    //had_error = true;
}

fn error(line: usize, message: &str)
{
    report(line, "", message);
}