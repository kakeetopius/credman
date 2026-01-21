use crate::util::errors::{CMError, CustomError};
use crate::util::ioutils;
use rusqlite::{Connection, ErrorCode, OpenFlags, Result};
use std::fs::exists;

pub fn get_db_con(dbfile: &str) -> Result<Connection, CMError> {
    let mut is_new_db: bool = false;
    let dbcon = match Connection::open_with_flags(&dbfile, OpenFlags::SQLITE_OPEN_READ_WRITE) {
        Ok(con) => con,
        Err(err) => {
            is_new_db = true;
            check_db_error(err, dbfile)?
        }
    };

    if !is_new_db {
        decrypt_db(&dbcon)?;
    }
    Ok(dbcon)
}

fn check_db_error(err: rusqlite::Error, db_path: &str) -> Result<Connection, CMError> {
    if let rusqlite::Error::SqliteFailure(e, _) = err
        && e.code == ErrorCode::CannotOpen
    {
        let opt = ioutils::get_terminal_input(
            "Could not find Database file. Do you want to initialise it(y/n)",
            false,
            false,
        )?;
        if opt.eq_ignore_ascii_case("y") {
            let con = create_new_db(db_path)?;
            return Ok(con);
        }
    }

    Err(CMError::RusqlilteError(err))
}

fn decrypt_db(dbcon: &Connection) -> Result<(), CMError> {
    let master_pass = ioutils::get_terminal_input("Enter master password", false, true)?;
    let pragma_query = format!("PRAGMA key = '{}';", &master_pass);
    dbcon.execute_batch(&pragma_query)?;

    let test_query = "SELECT COUNT(*) FROM sqlite_master";
    if let Err(err) = dbcon.execute_batch(&test_query) {
        if let rusqlite::Error::SqliteFailure(e, _) = err
            && e.code == ErrorCode::NotADatabase
        {
            return Err(CMError::Custom(CustomError::new(
                "\nCould not decrypt database. Please check the password and try again.",
            )));
        }
    }

    Ok(())
}

pub fn create_new_db(path: &str) -> Result<Connection, CMError> {
    if let Ok(true) = exists(path) {
        return Err(CustomError::new(&format!("File Already Exists at path: {}", path)).into());
    }
    let create_query = "CREATE TABLE account (\
	 acc_id INTEGER PRIMARY KEY AUTOINCREMENT,\
	 acc_name VARCHAR(100) NOT NULL UNIQUE,\
	 user_name VARCHAR(100),\
	 password VARCHAR(256)\
	);\
	CREATE TABLE api_keys (\
	api_id INTEGER PRIMARY KEY AUTOINCREMENT,\
	api_name VARCHAR(100) NOT NULL UNIQUE,\
	service VARCHAR(100),\
	user_name VARCHAR(100),\
	api_key VARCHAR(256)\
	);";

    let master_pass = ioutils::get_terminal_input(
        "Enter master password (Make sure to remember it)",
        true,
        true,
    )?;
    let pragma_query = format!("PRAGMA key = '{}';", &master_pass);
    let dbcon = Connection::open(path)?;

    dbcon.execute_batch(&pragma_query)?;
    dbcon.execute_batch(create_query)?;
    println!("Database Created at: {}", path);
    Ok(dbcon)
}

pub fn change_db_password(path: &String) -> Result<(), CMError> {
    let dbcon = Connection::open(path)?;
    decrypt_db(&dbcon)?;

    let master_pass = ioutils::get_terminal_input(
        "Enter new master password (Make sure to remember it)",
        true,
        true,
    )?;
    let pragma_query = format!("PRAGMA rekey = '{}';", &master_pass);

    dbcon.execute_batch(&pragma_query)?;
    Ok(())
}
