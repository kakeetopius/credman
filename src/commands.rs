use crate::db;
use crate::objects::{APIObj, AccountObj, Secret};
use crate::util::argparser::{
    AddArgs, ChangeArgs, CmanArgs, Commands, DeleteArgs, FieldType, GetArgs, InitArgs, LsArgs,
    SecretType,
};
use crate::util::errors::{CMError, CustomError};
use crate::util::ioutils::{
    get_multiple_selections_from_terminal, get_terminal_input, get_terminal_input_with_suggestions,
    get_user_confirmation,
};
use crate::util::passgen;

use clap::CommandFactory;
use clap_complete::generate;
use rusqlite::Connection;

use std::env::home_dir;
use std::env::var_os;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod add;
mod change;
mod delete;
mod get;

use get::get_account_from_user;
use get::get_api_from_user;

type Result = std::result::Result<(), CMError>;

const DB_ENV_VAR: &str = "CMAN_DBFILE";

pub fn run_command(args: &CmanArgs) -> Result {
    if let Commands::Init(args) = &args.command {
        return run_init(args);
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

    match &args.command {
        Commands::Add(a) => add::run_add(a, &dbcon),
        Commands::Get(a) => get::run_get(a, &dbcon),
        Commands::Change(a) => change::run_change(a, &dbcon),
        Commands::Delete(a) => delete::run_delete(a, &dbcon),
        Commands::Ls(a) => run_list(a, &dbcon),
        _ => Ok(()),
    }
}

fn run_init(args: &InitArgs) -> Result {
    let path = match &args.path {
        Some(p) => p.clone(),
        None => match get_db_path_from_env() {
            None => {
                return Err(CustomError::new(
                    "Could not get Database path. Try passing --path argument.",
                )
                .into());
            }
            Some(p) => p,
        },
    };

    db::create_new_db(&path)?;
    Ok(())
}

fn get_db_path_from_env() -> Option<String> {
    let path = var_os(DB_ENV_VAR).and_then(|v| v.into_string().ok());

    if let Some(credman_path) = path
        && !credman_path.is_empty()
    {
        return Some(credman_path);
    }

    let home = home_dir();
    if let Some(home_path) = home {
        let creds_path_buf = home_path.join(".creds.db");
        return Some(creds_path_buf.to_string_lossy().to_string());
    }

    None
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
