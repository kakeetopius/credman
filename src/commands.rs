use crate::db;
use crate::objects::{APIObj, AccountObj, Secret};
use crate::util::argparser::{
    AddArgs, ChangeArgs, CmanArgs, Commands, DeleteArgs, FieldType, GetArgs, InitArgs, LsArgs,
    SecretType,
};
use crate::util::errors::{CMError, CustomError};
use crate::util::ioutils::{
    get_terminal_input, get_terminal_input_with_suggestions, get_user_confirmation,
};
use crate::util::passgen;

use clap::CommandFactory;
use clap_complete::generate;
use rusqlite::Connection;

use std::env::var_os;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Result = std::result::Result<(), CMError>;

pub fn run_command(args: &CmanArgs) -> Result {
    if let Commands::Init(args) = &args.command {
        return run_init(&args);
    }
    if let Commands::Completions { shell } = &args.command {
        let mut cmd = CmanArgs::command();
        generate(*shell, &mut cmd, "cman", &mut std::io::stdout());
        return Ok(());
    }

    let dbpath = match get_db_path_from_env() {
        Some(p) => p,
        None => return Err(CustomError::new("Could not get Database file path").into()),
    };
    let dbcon = db::get_db_con(&dbpath)?;

    let res = match &args.command {
        Commands::Add(a) => run_add(&a, &dbcon),
        Commands::Get(a) => run_get(&a, &dbcon),
        Commands::Change(a) => run_change(&a, &dbcon),
        Commands::Delete(a) => run_delete(&a, &dbcon),
        Commands::Ls(a) => run_list(&a, &dbcon),
        _ => return Ok(()),
    };
    res
}

fn run_init(args: &InitArgs) -> Result {
    let path = match &args.path {
        Some(p) => p.clone(),
        None => {
            let path_from_env_vars = match get_db_path_from_env() {
                None => {
                    return Err(CustomError::new(
                        "Could not get Database path. Try passing --path argument.",
                    )
                    .into());
                }
                Some(p) => p,
            };
            path_from_env_vars
        }
    };

    if let Err(err) = db::create_new_db(&path) {
        return Err(err);
    }
    Ok(())
}

fn get_db_path_from_env() -> Option<String> {
    let path = var_os("CMAN_DBFILE").and_then(|v| v.into_string().ok());

    if let Some(credman_path) = path {
        if credman_path != "" {
            return Some(credman_path);
        }
    }

    let home = var_os("HOME").and_then(|v| v.into_string().ok());
    if let Some(home_path) = home {
        let home_path = Path::new(&home_path);
        let creds_path_buf = home_path.join(".creds.db");
        return Some(creds_path_buf.to_string_lossy().to_string());
    }

    return None;
}

fn get_account_from_user(dbcon: &Connection) -> core::result::Result<Secret, CMError> {
    let all_accounts = db::get_all_accounts_from_db(dbcon)?;
    if all_accounts.len() < 1 {
        return Err(CustomError::new(
            "No accounts added yet. Use cman add <account_name> to add your first account. See cman add --help for more details.",
        ).into());
    }
    get_terminal_input_with_suggestions("Enter the account name", "", all_accounts)
}

fn get_api_from_user(dbcon: &Connection) -> core::result::Result<Secret, CMError> {
    let all_api_keys = db::get_all_apikeys_from_db(dbcon)?;
    if all_api_keys.len() < 1 {
        return Err(CustomError::new(
            "No api keys added yet. Use cman add <api_name> to add your first api key. See cman add --help for more details.",
        ).into());
    }
    get_terminal_input_with_suggestions("Enter the api key name", "", all_api_keys)
}

fn run_get(args: &GetArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let secret = match &args.secret {
        Some(s) => match sec_type {
            SecretType::Login => db::get_account_from_db(&s, &dbcon)?,
            SecretType::Api => db::get_apikey_from_db(&s, &dbcon)?,
        },
        None => match sec_type {
            SecretType::Login => get_account_from_user(dbcon)?,
            SecretType::Api => get_api_from_user(dbcon)?,
        },
    };

    if let Some(fieldtype) = args.field {
        if args.json {
            secret.print_field_json(fieldtype);
        } else {
            secret.print_field(fieldtype);
        }
        return Ok(());
    }

    if args.json {
        secret.print_json();
        return Ok(());
    }
    secret.print();
    Ok(())
}

fn run_add(args: &AddArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let sec_name = &args.secret;
    if sec_name == "master" {
        return Err(CustomError::new(
            "Cannot use the name \"master\" because it is reserved for the master password",
        )
        .into());
    } else if args.batch {
        return add_secrets_from_batch(sec_name, args.passlen, dbcon);
    }

    match sec_type {
        SecretType::Login => add_new_acc(sec_name, args.passlen, args.no_auto, dbcon)?,
        SecretType::Api => add_new_api(sec_name, dbcon)?,
    };
    println!("Added Successfully");
    Ok(())
}

fn add_new_acc(name: &str, passlen: Option<usize>, noautopass: bool, dbcon: &Connection) -> Result {
    let exists = db::check_account_exists(name, dbcon)?;
    if exists {
        return Err(CustomError::new(&format!("Account {} already exists", name)).into());
    }
    let user_name = get_terminal_input("Enter username for the account", false, false)?;

    let pass = if noautopass {
        get_terminal_input("Enter Password", true, true)?
    } else {
        passgen::get_random_pass(passlen)?
    };

    db::add_account_to_db(
        &AccountObj {
            account_name: name.to_string(),
            user_name: user_name,
            password: pass,
        },
        dbcon,
    )?;
    Ok(())
}

fn add_new_api(name: &str, dbcon: &Connection) -> Result {
    let exists = db::check_apikey_exists(name, dbcon)?;
    if exists {
        return Err(CustomError::new(&format!("API Key {} already exists", name)).into());
    }
    let user_name = get_terminal_input(
        "Enter username for the account associated with API Key (if any)",
        false,
        false,
    )?;
    let desc = get_terminal_input("Enter a short description for the API key", false, false)?;
    let apikey = get_terminal_input("Enter API Key", false, false)?;

    db::add_apikey_to_db(
        &APIObj {
            api_name: name.to_string(),
            description: desc,
            user_name: user_name,
            api_key: apikey,
        },
        dbcon,
    )?;
    Ok(())
}

fn run_change(args: &ChangeArgs, dbcon: &Connection) -> Result {
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

fn run_delete(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let secret_type = args.secret_type.unwrap_or(SecretType::Login);

    match secret_type {
        SecretType::Login => delete_acc(args, dbcon)?,
        SecretType::Api => delete_api(args, dbcon)?,
    };
    Ok(())
}

fn delete_acc(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let sec_name = match &args.secret {
        Some(s) => s.clone(),
        None => {
            let acc_obj = get_account_from_user(dbcon)?;
            if let Secret::Account(acc) = acc_obj {
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
    let opt = get_user_confirmation(&format!(
        "Are you sure you want to delete {} (yes/no)",
        sec_name,
    ))?;
    if !opt {
        return Ok(());
    }
    db::delete_account_from_db(&sec_name, dbcon)?;
    println!("Account Deleted");
    Ok(())
}

fn delete_api(args: &DeleteArgs, dbcon: &Connection) -> Result {
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
    let opt = get_user_confirmation(&format!(
        "Are you sure you want to delete {} (yes/no)",
        sec_name
    ))?;
    if !opt {
        return Ok(());
    }
    db::delete_apikey_from_db(&sec_name, dbcon)?;
    println!("API Key Deleted");
    Ok(())
}

fn run_list(args: &LsArgs, dbcon: &Connection) -> Result {
    let secret_type = args.secret_type.unwrap_or(SecretType::Login);
    let results = match secret_type {
        SecretType::Login => db::get_all_accounts_from_db(dbcon),
        SecretType::Api => db::get_all_apikeys_from_db(dbcon),
    }?;

    if args.json {
        let json_str = serde_json::to_string_pretty(&results).unwrap_or("".to_string());
        println!("{}", json_str);
        return Ok(());
    }

    for result in results {
        result.print();
    }
    Ok(())
}

fn add_secrets_from_batch(batch_file: &str, passlen: Option<usize>, dbcon: &Connection) -> Result {
    let file = File::open(batch_file)?;
    let reader = BufReader::new(file);
    let mut lineno = 1;
    let mut errors_str = String::from("");
    let mut successfull: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line == "" {
            lineno += 1;
            continue;
        }
        let fields: Vec<_> = line.split(",").collect();

        if fields[0] == "login" {
            if fields.len() != 4 {
                errors_str.push_str(&format!("Line {}: Wrong number of fields\n", lineno));
                lineno += 1;
                continue;
            }
            let exists = db::check_account_exists(fields[1], dbcon)?;
            if exists {
                errors_str.push_str(&format!(
                    "Line {}: Account {} already exists\n",
                    lineno, fields[1]
                ));
                lineno += 1;
                continue;
            } else if fields[1] == "master" {
                errors_str.push_str(&format!(
                    "Line {}: Account name cannot be master.\n",
                    lineno
                ));
                lineno += 1;
                continue;
            } else if fields[1] == "" {
                errors_str.push_str(&format!("Line {}: Account name cannot be empty.\n", lineno));
                lineno += 1;
                continue;
            }
            let mut pass: String = fields[3].to_string();
            if pass == "?" {
                pass = passgen::get_random_pass(passlen)?;
            }
            let acc = AccountObj {
                account_name: fields[1].to_string(),
                user_name: fields[2].to_string(),
                password: pass,
            };
            match db::add_account_to_db(&acc, dbcon) {
                Err(e) => errors_str.push_str(&format!("Line {}: {}", lineno, e.to_string())),
                Ok(_) => successfull.push(fields[1].to_string()),
            }
        } else if fields[0] == "api" {
            if fields.len() != 5 {
                errors_str.push_str(&format!("Line {}: Wrong number of fields\n", lineno));
                lineno += 1;
                continue;
            }
            let exists = db::check_apikey_exists(fields[1], dbcon)?;
            if exists {
                errors_str.push_str(&format!(
                    "Line {}: API Key {} already exists\n",
                    lineno, fields[1]
                ));
                lineno += 1;
                continue;
            } else if fields[1] == "master" {
                errors_str.push_str(&format!("Line {}: Api name cannot be master.\n", lineno));
                lineno += 1;
                continue;
            } else if fields[1] == "" {
                errors_str.push_str(&format!("Line {}: Api name cannot be empty.\n", lineno));
                lineno += 1;
                continue;
            }
            let api = APIObj {
                api_name: fields[1].to_string(),
                user_name: fields[2].to_string(),
                description: fields[3].to_string(),
                api_key: fields[4].to_string(),
            };

            match db::add_apikey_to_db(&api, dbcon) {
                Err(e) => errors_str.push_str(&format!("Line {}: {}", lineno, e.to_string())),
                Ok(_) => successfull.push(fields[1].to_string()),
            }
        } else {
            errors_str.push_str(&format!(
                "Line {}: First field should be 'login' or 'api'\n",
                lineno
            ));
        }
        lineno += 1;
    }
    if errors_str != "" {
        println!("Got some errors:\n{}", errors_str);
        println!("Use cman add --help for more details");
    }
    if successfull.len() > 0 {
        println!("\nSuccessfully added:");
        for name in successfull {
            print!("{}, ", name);
        }
        println!();
    }
    Ok(())
}
