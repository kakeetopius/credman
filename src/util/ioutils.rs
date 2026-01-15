use std::io;
use std::io::Write;

pub fn get_terminal_input(prompt: &str) -> Result<String, io::Error> {
    print_prompt(prompt)?;
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(e);
    }

    Ok(input.trim().to_string())
}

fn print_prompt(prompt: &str) -> io::Result<()> {
    print!("{prompt}: ");
    io::stdout().flush()
}
