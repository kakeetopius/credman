use std::io;
use std::io::Write;

pub fn get_terminal_input() -> Result<String, io::Error> {
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(e);
    }

    Ok(input)
}

pub fn print_prompt(prompt: &str) -> io::Result<()> {
    print!("{prompt}: ");
    io::stdout().flush()
}
