use crate::cman_error;
use crate::db;
use crate::objects::{APIObj, AccountObj, Secret};
use crate::util::argparser::{
    AddArgs, ChangeArgs, CmanArgs, Commands, DeleteArgs, FieldType, GetArgs, InitArgs, LsArgs,
    PullArgs, SecretType,
};
use crate::util::errors::{CMError, CustomError};
use crate::util::ioutils::{
    self, get_multiple_selections_from_terminal, get_terminal_input,
    get_terminal_input_with_suggestions, get_user_confirmation,
};
use crate::util::passgen;

use clap::CommandFactory;
use clap_complete::generate;
use rusqlite::Connection;

use std::env::home_dir;
use std::env::var_os;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use get::get_account_from_user;
use get::get_api_from_user;

mod add;
mod change;
mod delete;
mod get;

type Result = std::result::Result<(), CMError>;

const DB_ENV_VAR: &str = "CMAN_DBFILE";
const REMOTE_DB_ENV_VAR: &str = "CMAN_DBURL";

pub fn run_command(args: &CmanArgs) -> Result {
    match &args.command {
        Commands::Init(args) => return run_init(args),
        Commands::Pull(args) => return run_pull(args),
        Commands::Completions { shell } => {
            let mut cmd = CmanArgs::command();
            generate(*shell, &mut cmd, "cman", &mut std::io::stdout());
            return Ok(());
        }
        _ => {}
    }

    let dbcon = {
        let dbpath = match get_db_path_from_env() {
            Some(p) => p,
            None => {
                return Err(cman_error!(&format!(
                    "Could not get Database file path. Try setting the {} environment variable.",
                    DB_ENV_VAR
                )));
            }
        };
        db::get_db_con(&dbpath)?
    };

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
                return Err(cman_error!(&format!(
                    "Could not get Database file path. Try setting the {} environment variable.",
                    DB_ENV_VAR
                )));
            }
            Some(p) => p,
        },
    };

    db::create_new_db(&path)?;
    Ok(())
}

fn run_list(args: &LsArgs, dbcon: &Connection) -> Result {
    let results = if args.all {
        let mut results = Vec::new();
        results.extend(db::get_all_accounts_from_db(dbcon)?);
        results.extend(db::get_all_apikeys_from_db(dbcon)?);

        results
    } else {
        let secret_type = args.secret_type.unwrap_or(SecretType::Login);
        match secret_type {
            SecretType::Login => db::get_all_accounts_from_db(dbcon)?,
            SecretType::Api => db::get_all_apikeys_from_db(dbcon)?,
        }
    };

    if args.json {
        let json = if args.pretty {
            serde_json::to_string_pretty(&results).unwrap_or("".to_string())
        } else {
            serde_json::to_string(&results).unwrap_or("".to_string())
        };
        println!("{}", json);
        return Ok(());
    }

    for result in results {
        result.print();
    }
    Ok(())
}

fn run_pull(args: &PullArgs) -> Result {
    let url = match &args.url {
        Some(u) => u.clone(),
        None => {
            let env_url = var_os(REMOTE_DB_ENV_VAR).and_then(|v| v.into_string().ok());
            match env_url {
                Some(u) => u,
                None => {
                    return Err(cman_error!(&format!(
                        "Could not determine remote url to use. Either provide it via the --url flag or set it using the {} environmnet variable.",
                        REMOTE_DB_ENV_VAR
                    )));
                }
            }
        }
    };

    let dbpath = match &args.out {
        Some(p) => p.clone(),
        None => match get_db_path_from_env() {
            Some(p) => p,
            None => {
                return Err(cman_error!(&format!(
                    "Could not determine where to save the database. Try setting the {} environmnet variable or pass the file path with the --out flag.",
                    DB_ENV_VAR
                )));
            }
        },
    };

    if Path::new(&dbpath).exists() {
        let opt = get_user_confirmation(&format!(
            "Are you sure you want to replace the credential database at {}",
            dbpath
        ))?;
        if !opt {
            return Ok(());
        }
    }

    let spinner = ioutils::new_spinner("Fetching Database......".into());

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send()?;

    let mut db = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(dbpath)?;

    std::io::copy(&mut response, &mut db)?;

    spinner.finish_with_message("Pull Done");
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
