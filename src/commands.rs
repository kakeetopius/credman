use crate::db;
use crate::objects::{APIObj, AccountObj};
use crate::util::argparser::{
    AddArgs, ChangeArgs, CmanArgs, Commands, DeleteArgs, FieldType, GetArgs, InitArgs, LsArgs,
    SecretType,
};
use crate::util::errors::{CMError, CustomError};
use crate::util::ioutils::get_terminal_input;
use crate::util::passgen;
use rusqlite::Connection;
use std::env::var_os;
use std::path::Path;

type Result = std::result::Result<(), CMError>;

pub fn run_command(args: &CmanArgs) -> Result {
    if let Commands::Init(args) = &args.command {
        return run_init(&args);
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
        Commands::Init(a) => run_init(&a),
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

fn run_get(args: &GetArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let result = match sec_type {
        SecretType::Login => db::get_account_from_db(&args.secret, &dbcon)?,
        SecretType::Api => db::get_apikey_from_db(&args.secret, &dbcon)?,
    };

    if let Some(fieldtype) = args.field {
        if args.json {
            result.print_field_json(fieldtype);
        } else {
            result.print_field(fieldtype);
        }
        return Ok(());
    }

    if args.json {
        result.print_json();
        return Ok(());
    }
    result.print();
    Ok(())
}

fn run_add(args: &AddArgs, dbcon: &Connection) -> Result {
    let field = args.secret_type.unwrap_or(SecretType::Login);
    match field {
        SecretType::Login => add_new_acc(&args.secret, args.no_auto, dbcon)?,
        SecretType::Api => add_new_api(&args.secret, dbcon)?,
    };
    println!("Added Successfully");
    Ok(())
}

fn add_new_acc(name: &str, noautopass: bool, dbcon: &Connection) -> Result {
    let exists = db::check_account_exists(name, dbcon)?;
    if exists {
        return Err(CustomError::new(&format!("Account {} already exists", name)).into());
    }
    let user_name = get_terminal_input("Enter username for the account", false, false)?;

    let pass = if noautopass {
        get_terminal_input("Enter Password", true, true)?
    } else {
        passgen::get_random_pass()?
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
    let service = get_terminal_input(
        "Enter the Service the API Key is for or a short description",
        false,
        false,
    )?;
    let apikey = get_terminal_input("Enter API Key", false, false)?;

    db::add_apikey_to_db(
        &APIObj {
            api_name: name.to_string(),
            api_service: service,
            user_name: user_name,
            api_key: apikey,
        },
        dbcon,
    )?;
    Ok(())
}

fn run_change(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let field = args.secret_type.unwrap_or(SecretType::Login);
    match field {
        SecretType::Login => change_acc_field(args, &dbcon)?,
        SecretType::Api => change_api_field(args, &dbcon)?,
    };
    println!("Changed Successfully");
    Ok(())
}

fn change_acc_field(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let exists = db::check_account_exists(&args.secret, dbcon)?;
    if !exists {
        return Err(CustomError::new(&format!("Account {} does not exist", args.secret)).into());
    }
    let fieldtype = args.field.unwrap_or(FieldType::Pass);
    let new_value = match fieldtype {
        FieldType::User => get_terminal_input("Enter new user name: ", false, false)?,
        FieldType::Secname => {
            get_terminal_input("Enter new name for the login credential", false, false)?
        }
        FieldType::Pass => {
            let opt = get_terminal_input(
                "Are you sure you want to change the password(y/n)",
                false,
                false,
            )?;
            if !opt.eq_ignore_ascii_case("y") {
                return Ok(());
            }
            let pass: String;
            if args.no_auto {
                pass = get_terminal_input("Enter new password", true, true)?
            } else {
                pass = passgen::get_random_pass()?;
            }
            pass
        }
        _ => {
            return Err(
                CustomError::new("The given field is invalid for a login credential").into(),
            );
        }
    };

    db::change_db_account_field(&args.secret, fieldtype, &new_value, dbcon)?;
    Ok(())
}

fn change_api_field(args: &ChangeArgs, dbcon: &Connection) -> Result {
    let exists = db::check_apikey_exists(&args.secret, dbcon)?;
    if !exists {
        return Err(CustomError::new(&format!("API {} does not exist", args.secret)).into());
    }
    let fieldtype = args.field.unwrap_or(FieldType::Key);
    let new_value = match fieldtype {
        FieldType::Secname => get_terminal_input("Enter new name for the api key", false, false)?,
        FieldType::Service => get_terminal_input(
            "Enter new service/description for the API key",
            false,
            false,
        )?,
        FieldType::User => get_terminal_input("Enter new user name", false, false)?,
        FieldType::Key => get_terminal_input("Enter new API key", false, false)?,
        _ => return Err(CustomError::new("The given field is invalid for an API key").into()),
    };

    db::change_db_apikey_field(&args.secret, fieldtype, &new_value, dbcon)?;
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
    let exists = db::check_account_exists(&args.secret, dbcon)?;
    if !exists {
        return Err(CustomError::new(&format!("Account {} does not exist", args.secret)).into());
    }
    let opt = get_terminal_input(
        &format!("Are you sure you want to delete {} (y/n)", args.secret),
        false,
        false,
    )?;
    if !opt.eq_ignore_ascii_case("y") {
        return Ok(());
    }
    db::delete_account_from_db(&args.secret, dbcon)?;
    Ok(())
}

fn delete_api(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let exists = db::check_apikey_exists(&args.secret, dbcon)?;
    if !exists {
        return Err(CustomError::new(&format!("API {} does not exist", args.secret)).into());
    }
    let opt = get_terminal_input(
        &format!("Are you sure you want to delete {} (y/n)", args.secret),
        false,
        false,
    )?;
    if !opt.eq_ignore_ascii_case("y") {
        return Ok(());
    }
    db::delete_apikey_from_db(&args.secret, dbcon)?;
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
