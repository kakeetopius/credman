use anyhow::anyhow;
use rusqlite::Connection;

use crate::{
    commands::{
        Result,
        get::{get_account_from_user, get_key_from_user},
    },
    db,
    objects::Secret,
    util::{
        argparser::{ChangeArgs, FieldType, SecretType},
        ioutils::{get_terminal_input, get_user_confirmation},
        passgen,
    },
};

pub fn run_change(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    if let Some(s) = &args.secret
        && s == "master"
    {
        db::change_db_password(dbcon)?;
        println!("Master Password Changed Successfully");
        return Ok(());
    }

    match sec_type {
        SecretType::Login => change_acc_field(args, dbcon)?,
        SecretType::Key => change_key_field(args, dbcon)?,
    };
    Ok(())
}

fn change_acc_field(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let sec_name = match &args.secret {
        Some(s) => s.clone(),
        None => {
            let account = get_account_from_user(dbcon)?;
            if let Secret::Login(acc) = account {
                acc.account_name.clone()
            } else {
                "".to_string()
            }
        }
    };

    let exists = db::check_account_exists(&sec_name, dbcon)?;
    if !exists {
        return Err(anyhow!(format!("Account '{}' does not exist", sec_name)));
    }
    let fieldtype = args.field.unwrap_or(FieldType::Pass);
    let new_value = match fieldtype {
        FieldType::User => get_terminal_input("Enter new user name", false, false)?,
        FieldType::Secname => {
            let input =
                get_terminal_input("Enter new name for the login credential", false, false)?;
            let exists = db::check_account_exists(&input, dbcon)?;
            if exists {
                return Err(anyhow!(format!(
                    "Account with name '{}' already exists",
                    input
                )));
            }
            if input == "master" {
                return Err(anyhow!(
                    "Cannot change name to \"master\" because it reserved for master password."
                ));
            }
            input
        }
        FieldType::Pass => {
            let opt =
                get_user_confirmation("Are you sure you want to change the password(yes/no)")?;

            if !opt {
                return Ok(());
            }

            if args.no_auto {
                get_terminal_input("Enter new password", true, true)?
            } else {
                passgen::get_random_pass(args.passlen)?
            }
        }
        _ => {
            return Err(anyhow!(format!(
                "The field '{}' is invalid for a login credential",
                fieldtype
            )));
        }
    };

    db::change_db_account_field(&sec_name, fieldtype, &new_value, dbcon)?;
    println!("Changed Successfully");
    Ok(())
}

fn change_key_field(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let sec_name = match &args.secret {
        Some(s) => s.clone(),
        None => {
            let keyobj = get_key_from_user(dbcon)?;
            if let Secret::Key(key) = keyobj {
                key.key_name.clone()
            } else {
                "".to_string()
            }
        }
    };
    let exists = db::check_key_exists(&sec_name, dbcon)?;
    if !exists {
        return Err(anyhow!(format!("Key '{}' does not exist", sec_name)));
    }
    let fieldtype = args.field.unwrap_or(FieldType::Key);
    let new_value = match fieldtype {
        FieldType::Secname => {
            let input = get_terminal_input("Enter new name for the key", false, false)?;
            let exists = db::check_key_exists(&input, dbcon)?;
            if exists {
                return Err(anyhow!(format!(
                    "Key with name '{}' already exists",
                    sec_name
                )));
            }
            if input == "master" {
                return Err(anyhow!(
                    "Cannot change name to \"master\" because it reserved for master password."
                ));
            }
            input
        }
        FieldType::Desc => {
            get_terminal_input("Enter new description for the Key key", false, false)?
        }
        FieldType::User => get_terminal_input("Enter new user name", false, false)?,
        FieldType::Key => get_terminal_input("Enter the new value of the key", false, false)?,
        _ => {
            return Err(anyhow!(format!(
                "The '{}' field is invalid for a key",
                fieldtype
            )));
        }
    };

    db::change_db_key_field(&sec_name, fieldtype, &new_value, dbcon)?;
    println!("Changed Successfully");
    Ok(())
}
