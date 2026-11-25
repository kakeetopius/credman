use anyhow::anyhow;
use rusqlite::Connection;

use crate::{
    commands::{Result, get},
    db,
    util::{
        argparser::{DeleteArgs, SecretType},
        ioutils::get_user_confirmation,
    },
};

pub fn run_delete(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let secret_type = args.secret_type.unwrap_or(SecretType::Login);

    match secret_type {
        SecretType::Login => delete_acc(args, dbcon)?,
        SecretType::Key => delete_key(args, dbcon)?,
    };
    Ok(())
}

fn delete_acc(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let accounts = match &args.secret {
        Some(accounts) => accounts.clone(),
        None => {
            let accounts = if args.multiple {
                get::get_accounts_from_user(dbcon)?
            } else {
                vec![get::get_account_from_user(dbcon)?]
            };

            accounts.iter().map(|secret| secret.get_name()).collect()
        }
    };

    let mut error_str = String::new();
    let mut successfull: Vec<String> = Vec::new();
    for account in accounts {
        let exists = db::check_account_exists(&account, dbcon)?;
        if !exists {
            error_str.push_str(&format!("Account '{}' does not exist\n", account));
            continue;
        }
        let opt = get_user_confirmation(&format!(
            "Are you sure you want to delete '{}' (yes/no)",
            account
        ))?;
        if !opt {
            continue;
        }
        db::delete_account_from_db(&account, dbcon)?;
        successfull.push(account)
    }

    if !successfull.is_empty() {
        println!("\nSuccessfully deleted:");
        for name in successfull {
            print!("{} ", name);
        }
        println!();
    }
    if !error_str.is_empty() {
        return Err(anyhow!(error_str));
    }
    Ok(())
}

fn delete_key(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let keys = match &args.secret {
        Some(keys) => keys.clone(),
        None => {
            let gotkeys = if args.multiple {
                get::get_keys_from_user(dbcon)?
            } else {
                vec![get::get_key_from_user(dbcon)?]
            };
            gotkeys.iter().map(|secret| secret.get_name()).collect()
        }
    };

    let mut error_str = String::new();
    let mut successfull: Vec<String> = Vec::new();
    for key in keys {
        let exists = db::check_key_exists(&key, dbcon)?;
        if !exists {
            error_str.push_str(&format!("Key '{}' does not exist\n", key));
            continue;
        }
        let opt = get_user_confirmation(&format!(
            "Are you sure you want to delete '{}' (yes/no)",
            key
        ))?;
        if !opt {
            continue;
        }
        db::delete_key_from_db(&key, dbcon)?;
        successfull.push(key)
    }

    if !successfull.is_empty() {
        println!("\nSuccessfully deleted:");
        for name in successfull {
            print!("{} ", name);
        }
        println!();
    }
    if !error_str.is_empty() {
        return Err(anyhow!(error_str));
    }
    Ok(())
}
