use crate::commands::*;

pub fn run_change(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    if let Some(s) = &args.secret {
        if s == "master" {
            db::change_db_password(dbcon)?;
            println!("Master Password Changed Successfully");
            return Ok(());
        }
    }

    match sec_type {
        SecretType::Login => change_acc_field(args, &dbcon)?,
        SecretType::Api => change_api_field(args, &dbcon)?,
    };
    Ok(())
}

fn change_acc_field(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let sec_name = match &args.secret {
        Some(s) => s.clone(),
        None => {
            let account = get_account_from_user(dbcon)?;
            if let Secret::Account(acc) = account {
                acc.account_name.clone()
            } else {
                "".to_string()
            }
        }
    };

    let exists = db::check_account_exists(&sec_name, dbcon)?;
    if !exists {
        return Err(CustomError::new(&format!("Account {} does not exist", sec_name)).into());
    }
    let fieldtype = args.field.unwrap_or(FieldType::Pass);
    let new_value = match fieldtype {
        FieldType::User => get_terminal_input("Enter new user name", false, false)?,
        FieldType::Secname => {
            let input =
                get_terminal_input("Enter new name for the login credential", false, false)?;
            let exists = db::check_account_exists(&input, dbcon)?;
            if exists {
                return Err(CustomError::new(&format!(
                    "Account with name {} already exists",
                    input
                ))
                .into());
            }
            if input == "master" {
                return Err(CustomError::new(
                    "Cannot change name to \"master\" because it reserved for master password.",
                )
                .into());
            }
            input
        }
        FieldType::Pass => {
            let opt =
                get_user_confirmation("Are you sure you want to change the password(yes/no)")?;

            if !opt {
                return Ok(());
            }
            let pass: String;
            if args.no_auto {
                pass = get_terminal_input("Enter new password", true, true)?
            } else {
                pass = passgen::get_random_pass(args.passlen)?;
            }
            pass
        }
        _ => {
            return Err(
                CustomError::new("The given field is invalid for a login credential").into(),
            );
        }
    };

    db::change_db_account_field(&sec_name, fieldtype, &new_value, dbcon)?;
    println!("Changed Successfully");
    Ok(())
}

fn change_api_field(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let sec_name = match &args.secret {
        Some(s) => s.clone(),
        None => {
            let api_obj = get_api_from_user(dbcon)?;
            if let Secret::API(api) = api_obj {
                api.api_name.clone()
            } else {
                "".to_string()
            }
        }
    };
    let exists = db::check_apikey_exists(&sec_name, dbcon)?;
    if !exists {
        return Err(CustomError::new(&format!("API {} does not exist", sec_name)).into());
    }
    let fieldtype = args.field.unwrap_or(FieldType::Key);
    let new_value = match fieldtype {
        FieldType::Secname => {
            let input = get_terminal_input("Enter new name for the api key", false, false)?;
            let exists = db::check_apikey_exists(&input, dbcon)?;
            if !exists {
                return Err(CustomError::new(&format!(
                    "API with name {} already exists",
                    sec_name
                ))
                .into());
            }
            if input == "master" {
                return Err(CustomError::new(
                    "Cannot change name to \"master\" because it reserved for master password.",
                )
                .into());
            }
            input
        }
        FieldType::Desc => {
            get_terminal_input("Enter new description for the API key", false, false)?
        }
        FieldType::User => get_terminal_input("Enter new user name", false, false)?,
        FieldType::Key => get_terminal_input("Enter new API key", false, false)?,
        _ => return Err(CustomError::new("The given field is invalid for an API key").into()),
    };

    db::change_db_apikey_field(&sec_name, fieldtype, &new_value, dbcon)?;
    println!("Changed Successfully");
    Ok(())
}
