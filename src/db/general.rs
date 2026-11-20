use std::fs::exists;

use anyhow::anyhow;
use rusqlite::{Connection, ErrorCode, OpenFlags};

use crate::util::ioutils;

pub fn get_db_con(dbfile: &str, master_pass: Option<String>) -> anyhow::Result<Connection> {
    let mut is_new_db: bool = false;
    let dbcon = match Connection::open_with_flags(dbfile, OpenFlags::SQLITE_OPEN_READ_WRITE) {
        Ok(con) => con,
        Err(err) => {
            is_new_db = true;
            check_db_error(err, dbfile)?
        }
    };

    if !is_new_db {
        decrypt_db(&dbcon, master_pass)?;
    }
    Ok(dbcon)
}

fn check_db_error(err: rusqlite::Error, db_path: &str) -> anyhow::Result<Connection> {
    if let rusqlite::Error::SqliteFailure(e, _) = err
        && e.code == ErrorCode::CannotOpen
    {
        let opt = ioutils::get_user_confirmation(
            "Could not find Database file. Do you want to initialise it",
        )?;
        if opt {
            let con = create_new_db(db_path)?;
            return Ok(con);
        }
    }

    Err(err.into())
}

fn decrypt_db(dbcon: &Connection, pass: Option<String>) -> anyhow::Result<()> {
    const MAX_TRIES: i32 = 3;

    let mut tries = 0;
    let mut failed = true;

    let pragma_query = "PRAGMA cipher_log = 'off';".to_string();
    dbcon.execute_batch(&pragma_query)?;

    while failed && tries < MAX_TRIES {
        let prompt = if tries == 0 {
            "Enter cman master password"
        } else {
            "Wrong Password. Please Try again"
        };

        let master_pass = match &pass {
            Some(pass) => pass,
            None => &ioutils::get_terminal_input(prompt, false, true)?,
        };

        if master_pass.is_empty() {
            return Err(anyhow!("Master password cannot be empty"));
        }

        let decryption_query = format!("PRAGMA key = '{}';", master_pass);
        dbcon.execute_batch(&decryption_query)?;

        let test_query = "SELECT COUNT(*) FROM sqlite_master";
        match dbcon.execute_batch(test_query) {
            Err(err) => match err {
                rusqlite::Error::SqliteFailure(e, _) if e.code == ErrorCode::NotADatabase => {
                    // this error is most likely caused if a wrong password given
                    tries += 1;
                    continue;
                }
                _ => return Err(err.into()), // any other error we return immediately.
            },

            Ok(()) => failed = false,
        }
    }

    if failed {
        return Err(anyhow!(
            "Could not decrypt database. Please check the password and try again."
        ));
    }

    Ok(())
}

pub fn create_new_db(path: &str) -> anyhow::Result<Connection> {
    if let Ok(true) = exists(path) {
        return Err(anyhow!(format!("File Already Exists at path: {}", path)));
    }

    let create_query = r#"
    CREATE TABLE account (
        acc_id INTEGER PRIMARY KEY AUTOINCREMENT,
        acc_name VARCHAR(100) NOT NULL UNIQUE,
        user_name VARCHAR(100),
        password VARCHAR(256),
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        password_last_changed DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE api_keys (
        api_id INTEGER PRIMARY KEY AUTOINCREMENT,
        api_name VARCHAR(100) NOT NULL UNIQUE,
        description VARCHAR(100),
        user_name VARCHAR(100),
        api_key VARCHAR(256),
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        api_key_last_changed DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TRIGGER account_password_last_changed
    AFTER UPDATE OF password ON account
    FOR EACH ROW
    WHEN OLD.password IS NOT NEW.password
    BEGIN
        UPDATE account
        SET password_last_changed = CURRENT_TIMESTAMP
        WHERE acc_id = OLD.acc_id;
    END;

    CREATE TRIGGER api_key_last_changed
    AFTER UPDATE OF api_key ON api_keys
    FOR EACH ROW
    WHEN OLD.api_key IS NOT NEW.api_key
    BEGIN
        UPDATE api_keys
        SET api_key_last_changed = CURRENT_TIMESTAMP
        WHERE api_id = OLD.api_id;
    END;
    "#;

    let master_pass = ioutils::get_terminal_input(
        "Enter master password (Make sure to remember it)",
        true,
        true,
    )?;
    if master_pass.is_empty() {
        return Err(anyhow!("Master password cannot be empty"));
    }
    let pragma_query = format!("PRAGMA key = '{}';", master_pass);
    let dbcon = Connection::open(path)?;

    dbcon.execute_batch(&pragma_query)?;
    dbcon.execute_batch(create_query)?;
    println!("Database Created at: {}", path);
    Ok(dbcon)
}

pub fn change_db_password(dbcon: &Connection) -> anyhow::Result<()> {
    let master_pass = ioutils::get_terminal_input(
        "Enter new master password (Make sure to remember it)",
        true,
        true,
    )?;
    if master_pass.is_empty() {
        return Err(anyhow!("Master password cannot be empty"));
    }
    let pragma_query = format!("PRAGMA rekey = '{}';", master_pass);

    dbcon.execute_batch(&pragma_query)?;
    Ok(())
}
