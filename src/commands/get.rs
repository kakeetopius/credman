use anyhow::anyhow;
use rusqlite::Connection;
use serde_json;

use crate::{
    commands::Result,
    db,
    objects::Secret,
    util::{
        argparser::{GetArgs, SecretType},
        ioutils::{
            self, get_multiple_selections_from_terminal, get_terminal_input_with_suggestions,
        },
    },
};

pub fn run_get(args: &GetArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let secrets = match &args.secret {
        Some(s) => match sec_type {
            SecretType::Login => get_accounts(s, dbcon)?,
            SecretType::Key => get_keys(s, dbcon)?,
        },
        None => match sec_type {
            SecretType::Login => {
                if args.multiple {
                    get_accounts_from_user(dbcon)?
                } else {
                    vec![get_account_from_user(dbcon)?]
                }
            }
            SecretType::Key => {
                if args.multiple {
                    get_keys_from_user(dbcon)?
                } else {
                    vec![get_key_from_user(dbcon)?]
                }
            }
        },
    };

    if secrets.is_empty() {
        return Ok(());
    }

    // if user requires json we combine everything in a single json object.
    if args.json {
        if let Some(fieldtype) = args.field {
            let secrets_fields: Vec<String> =
                secrets.iter().map(|s| s.get_field(fieldtype)).collect();

            let json = if args.pretty {
                serde_json::to_string_pretty(&secrets_fields).unwrap_or("".to_string())
            } else {
                serde_json::to_string(&secrets_fields).unwrap_or("".to_string())
            };
            println!("{}", json);
            return Ok(());
        }

        let json = if args.pretty {
            serde_json::to_string_pretty(&secrets).unwrap_or("".to_string())
        } else {
            serde_json::to_string(&secrets).unwrap_or("".to_string())
        };
        println!("{}", json);
        return Ok(());
    }

    ioutils::print_secrets(&secrets, args)?;
    Ok(())
}

fn get_accounts(accounts: &Vec<String>, dbcon: &Connection) -> anyhow::Result<Vec<Secret>> {
    let mut account_objs: Vec<Secret> = Vec::new();
    let mut errors: Vec<anyhow::Error> = Vec::new();

    for account in accounts {
        let account = db::get_account_from_db(account, dbcon);
        match account {
            Ok(a) => account_objs.push(a),
            Err(e) => errors.push(e),
        }
    }
    if !errors.is_empty() {
        for e in errors {
            eprintln!("{}", e);
        }
        eprintln!();
    }
    Ok(account_objs)
}

fn get_keys(keys: &Vec<String>, dbcon: &Connection) -> anyhow::Result<Vec<Secret>> {
    let mut key_objs: Vec<Secret> = Vec::new();
    let mut errors: Vec<anyhow::Error> = Vec::new();

    for key in keys {
        let got_key = db::get_key_from_db(key, dbcon);
        match got_key {
            Ok(key) => key_objs.push(key),
            Err(e) => errors.push(e),
        }
    }
    if !errors.is_empty() {
        for e in errors {
            eprintln!("{}", e);
        }
        eprintln!();
    }
    Ok(key_objs)
}

pub fn get_account_from_user(dbcon: &Connection) -> anyhow::Result<Secret> {
    let all_accounts = db::get_all_accounts_from_db(dbcon)?;
    if all_accounts.is_empty() {
        return Err(anyhow!(
            "No accounts added yet. Use cman add <account_name> to add your first account. See cman add --help for more details."
        ));
    }
    get_terminal_input_with_suggestions("Enter the account name", all_accounts)
}

pub fn get_key_from_user(dbcon: &Connection) -> anyhow::Result<Secret> {
    let all_keys = db::get_all_keys_from_db(dbcon)?;
    if all_keys.is_empty() {
        return Err(anyhow!(
            "No keys added yet. Use cman add <key_name> to add your first key. See cman add --help for more details."
        ));
    }
    get_terminal_input_with_suggestions("Enter the key name", all_keys)
}

pub fn get_accounts_from_user(dbcon: &Connection) -> anyhow::Result<Vec<Secret>> {
    let all_accounts = db::get_all_accounts_from_db(dbcon)?;
    if all_accounts.is_empty() {
        return Err(anyhow!(
            "No accounts added yet. Use cman add <account_name> to add your first account. See cman add --help for more details."
        ));
    }

    get_multiple_selections_from_terminal("Select accounts", all_accounts)
}

pub fn get_keys_from_user(dbcon: &Connection) -> anyhow::Result<Vec<Secret>> {
    let all_keys = db::get_all_keys_from_db(dbcon)?;
    if all_keys.is_empty() {
        return Err(anyhow!(
            "No keys added yet. Use cman add <key_name> to add your first key. See cman add --help for more details."
        ));
    }
    get_multiple_selections_from_terminal("Select Keys", all_keys)
}
