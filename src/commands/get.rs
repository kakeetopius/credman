use crate::commands::*;
use serde_json;

pub fn run_get(args: &GetArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let secrets = match &args.secret {
        Some(s) => match sec_type {
            SecretType::Login => get_accounts(s, dbcon)?,
            SecretType::Api => get_apikeys(s, dbcon)?,
        },
        None => match sec_type {
            SecretType::Login => {
                if args.multiple {
                    get_multiple_accounts_from_user(dbcon)?
                } else {
                    vec![get_account_from_user(dbcon)?]
                }
            }
            SecretType::Api => {
                if args.multiple {
                    get_multiple_apikeys_from_user(dbcon)?
                } else {
                    vec![get_api_from_user(dbcon)?]
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
            if secrets.len() == 1 {
                println!("{}", secrets[0].get_field_json_str(fieldtype));
                return Ok(());
            }

            let secrets_fields: Vec<String> =
                secrets.iter().map(|s| s.get_field(fieldtype)).collect();
            let json = match serde_json::to_string_pretty(&secrets_fields) {
                Err(_) => "".to_string(),
                Ok(j) => j,
            };
            println!("{}", json);
            return Ok(());
        }

        if secrets.len() == 1 {
            println!("{}", secrets[0].get_json_str());
            return Ok(());
        }
        let json = match serde_json::to_string_pretty(&secrets) {
            Err(_) => "".to_string(),
            Ok(j) => j,
        };
        println!("{}", json);
        return Ok(());
    }

    println!();
    for secret in secrets {
        if let Some(fieldtype) = args.field {
            secret.print_field(fieldtype);
            continue;
        }
        secret.print();
    }
    Ok(())
}

fn get_accounts(
    accounts: &Vec<String>,
    dbcon: &Connection,
) -> core::result::Result<Vec<Secret>, CMError> {
    let mut account_objs: Vec<Secret> = Vec::new();
    let mut errors: Vec<CMError> = Vec::new();

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

fn get_apikeys(
    apikeys: &Vec<String>,
    dbcon: &Connection,
) -> core::result::Result<Vec<Secret>, CMError> {
    let mut api_objs: Vec<Secret> = Vec::new();
    let mut errors: Vec<CMError> = Vec::new();

    for apikey in apikeys {
        let apikey = db::get_apikey_from_db(apikey, dbcon);
        match apikey {
            Ok(api) => api_objs.push(api),
            Err(e) => errors.push(e),
        }
    }
    if !errors.is_empty() {
        for e in errors {
            eprintln!("{}", e);
        }
        eprintln!();
    }
    Ok(api_objs)
}

pub fn get_account_from_user(dbcon: &Connection) -> core::result::Result<Secret, CMError> {
    let all_accounts = db::get_all_accounts_from_db(dbcon)?;
    if all_accounts.is_empty() {
        return Err(CustomError::new(
            "No accounts added yet. Use cman add <account_name> to add your first account. See cman add --help for more details.",
        ).into());
    }
    get_terminal_input_with_suggestions("Enter the account name", all_accounts)
}

pub fn get_api_from_user(dbcon: &Connection) -> core::result::Result<Secret, CMError> {
    let all_api_keys = db::get_all_apikeys_from_db(dbcon)?;
    if all_api_keys.is_empty() {
        return Err(CustomError::new(
            "No api keys added yet. Use cman add <api_name> to add your first api key. See cman add --help for more details.",
        ).into());
    }
    get_terminal_input_with_suggestions("Enter the api key name", all_api_keys)
}

pub fn get_multiple_accounts_from_user(
    dbcon: &Connection,
) -> core::result::Result<Vec<Secret>, CMError> {
    let all_accounts = db::get_all_accounts_from_db(dbcon)?;
    if all_accounts.is_empty() {
        return Err(CustomError::new(
            "No accounts added yet. Use cman add <account_name> to add your first account. See cman add --help for more details.",
        ).into());
    }

    get_multiple_selections_from_terminal("Select accounts", all_accounts)
}

pub fn get_multiple_apikeys_from_user(
    dbcon: &Connection,
) -> core::result::Result<Vec<Secret>, CMError> {
    let all_api_keys = db::get_all_apikeys_from_db(dbcon)?;
    if all_api_keys.is_empty() {
        return Err(CustomError::new(
            "No api keys added yet. Use cman add <api_name> to add your first api key. See cman add --help for more details.",
        ).into());
    }
    get_multiple_selections_from_terminal("Select API Keys", all_api_keys)
}
