use crate::util::errors::{CMError, CustomError};
use std::io;
use std::io::Write;

pub fn get_terminal_input(prompt: &str, confirm: bool, _private: bool) -> Result<String, CMError> {
    let quiet = false;
    if !quiet {
        print_prompt(prompt)?;
    }
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(e.into());
    }

    if confirm {
        if !quiet {
            print_prompt("Enter again to confirm")?;
        }
        let mut input_2 = String::new();
        if let Err(e) = io::stdin().read_line(&mut input_2) {
            return Err(e.into());
        }

        if input != input_2 {
            return Err(CustomError::new("Inputs do not match").into());
        }
    }

    Ok(input.trim().to_string())
}

fn print_prompt(prompt: &str) -> io::Result<()> {
    print!("{prompt}: ");
    io::stdout().flush()
}

pub fn print_result(field: &str, value: &str) {
    println!("{}:   {}", field, value);
}
