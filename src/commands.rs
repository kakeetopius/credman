use std::{
    env::{home_dir, var_os},
    path::Path,
};

use anyhow::{Context, anyhow};
use clap::CommandFactory;
use clap_complete::generate;
use rusqlite::Connection;

use crate::{
    db,
    util::{
        argparser::{CmanArgs, Commands, InitArgs, LsArgs, PullArgs, SecretType},
        ioutils,
    },
};

mod add;
mod change;
mod delete;
mod export;
mod get;

type Result = std::result::Result<(), anyhow::Error>;

const DB_ENV_VAR: &str = "CMAN_DBFILE";
const REMOTE_DB_ENV_VAR: &str = "CMAN_DBURL";
const CMAN_PASSWD_FILE: &str = "CMAN_PASSWD_FILE";

pub fn run_command(args: CmanArgs) -> Result {
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
                return Err(anyhow!(format!(
                    "Could not get Database file path. Try setting the {} environment variable.",
                    DB_ENV_VAR
                )));
            }
        };

        db::get_db_con(&dbpath, try_get_decryption_pass(args.passwd_file)?)?
    };

    match &args.command {
        Commands::Add(a) => add::run_add(a, &dbcon),
        Commands::Get(a) => get::run_get(a, &dbcon),
        Commands::Change(a) => change::run_change(a, &dbcon),
        Commands::Delete(a) => delete::run_delete(a, &dbcon),
        Commands::Ls(a) => run_list(a, &dbcon),
        Commands::Export(a) => export::run_export(a, &dbcon),
        _ => Ok(()),
    }
}

fn run_init(args: &InitArgs) -> Result {
    let path = match &args.path {
        Some(p) => p.clone(),
        None => match get_db_path_from_env() {
            None => {
                return Err(anyhow!(format!(
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
                    return Err(anyhow!(format!(
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
                return Err(anyhow!(format!(
                    "Could not determine where to save the database. Try setting the {} environmnet variable or pass the file path with the --out flag.",
                    DB_ENV_VAR
                )));
            }
        },
    };

    if Path::new(&dbpath).exists() {
        let opt = ioutils::get_user_confirmation(&format!(
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

fn try_get_decryption_pass(file_name: Option<String>) -> anyhow::Result<Option<String>> {
    let file = match file_name {
        Some(f) => f,
        None => {
            let env = var_os(CMAN_PASSWD_FILE).and_then(|v| v.into_string().ok());
            match env {
                Some(p) => p,
                None => return Ok(None), // could not get password file from file_name or from env
                                         // so decryption func will prompt user.
            }
        }
    };

    let content = std::fs::read_to_string(&file)
        .context(format!("Could not open the password file {file}"))?;

    if let Some(pass) = content.lines().next() {
        return Ok(Some(pass.to_owned()));
    }

    Err(anyhow!("The password file contains no password"))
}
