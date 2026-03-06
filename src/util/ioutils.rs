use crate::objects::Secret;
use crate::util::argparser::{CmanArgs, Commands, FieldType, GetArgs, SecretType};
use crate::util::errors::CMError;

use inquire::*;
use std::fmt::Display;
use std::sync::Mutex;

static QUIET: Mutex<bool> = Mutex::new(false);

pub fn get_terminal_input(prompt: &str, confirm: bool, private: bool) -> Result<String, CMError> {
    if private {
        return get_private_input(prompt, confirm);
    }
    let prompt = &format!("{}: ", prompt);

    let input = Text::new(prompt).prompt()?;
    Ok(input)
}

pub fn get_terminal_input_with_suggestions<T>(
    prompt: &str,
    suggestions: Vec<T>,
) -> Result<T, CMError>
where
    T: Display,
{
    let prompt = &format!("{}: ", prompt);

    let option = Select::new(prompt, suggestions).prompt()?;

    Ok(option)
}

pub fn get_multiple_selections_from_terminal<T>(
    prompt: &str,
    options: Vec<T>,
) -> Result<Vec<T>, CMError>
where
    T: Display,
{
    let prompt = &format!("{}: ", prompt);

    let options = MultiSelect::new(prompt, options).prompt()?;
    Ok(options)
}

fn get_private_input(prompt: &str, confirm: bool) -> Result<String, CMError> {
    let prompt = &format!("{}: ", prompt);

    let mut password = Password::new(prompt);
    if !confirm {
        password = password.without_confirmation()
    }

    let input_password = password.prompt()?;
    Ok(input_password)
}

pub fn get_user_confirmation(message: &str) -> Result<bool, CMError> {
    let prompt = &format!("{}: ", message);
    let ans = Confirm::new(prompt).prompt()?;
    Ok(ans)
}

pub fn print_result(field: &str, value: &str) {
    let quiet = shouldbequiet();
    if !quiet {
        print!("{}:   ", field);
    }
    println!("{}", value);
}

pub fn print_secrets(secrets: &Vec<Secret>, getargs: &GetArgs) {
    let sec_type = getargs.secret_type.unwrap_or(SecretType::Login);
    let _default_field = if sec_type == SecretType::Login {
        FieldType::Pass
    } else {
        FieldType::Key
    };

    let quiet = shouldbequiet();
    if !quiet {
        println!()
    }

    for secret in secrets {
        if let Some(fieldtype) = getargs.field {
            secret.print_field(fieldtype);
            continue;
        }
        secret.print();
    }
}

pub fn set_terminal_settings(args: &CmanArgs) {
    if let Commands::Get(getargs) = &args.command
        && getargs.quiet
    {
        let guard = QUIET.lock().ok();
        if let Some(mut guard) = guard {
            *guard = true;
        }
    }
}

fn shouldbequiet() -> bool {
    let guard = QUIET.lock().ok();
    match guard {
        Some(guard) => *guard,
        None => false,
    }
}
