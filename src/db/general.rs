use crate::util::errors;
use crate::util::ioutils;
use rusqlite::{Connection, Error, ErrorCode, OpenFlags, Result};

pub fn get_db_con() -> Result<Connection, Box<dyn std::error::Error>> {
    let dbfile = "/home/pius/.creds.db".to_string();
    let mut is_new_db: bool = false;
    let dbcon = match Connection::open_with_flags(&dbfile, OpenFlags::SQLITE_OPEN_READ_WRITE) {
        Ok(con) => con,
        Err(err) => {
            is_new_db = true;
            check_db_error(err, &dbfile)?
        }
    };

    if !is_new_db {
        decrypt_db(&dbcon)?;
    }
    println!("Done decrypting");
    Ok(dbcon)
}

fn check_db_error(err: Error, db_path: &String) -> Result<Connection, Box<dyn std::error::Error>> {
    if let Error::SqliteFailure(e, _) = err
        && e.code == ErrorCode::CannotOpen
    {
        let opt = ioutils::get_terminal_input(
            "Could not find Database file. Do you want to initialise it(y/n)",
        )?;
        if opt.eq_ignore_ascii_case("y") {
            let con = create_new_db(db_path)?;
            return Ok(con);
        }
    }

    Err(Box::new(err))
}

fn decrypt_db(dbcon: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let master_pass = ioutils::get_terminal_input("Enter master password")?;
    let pragma_query = format!("PRAGMA key = '{}';", &master_pass);
    dbcon.execute_batch(&pragma_query)?;

    let test_query = "SELECT COUNT(*) FROM sqlite_master";
    if let Err(err) = dbcon.execute_batch(&test_query) {
        if let Error::SqliteFailure(e, _) = err
            && e.code == ErrorCode::NotADatabase
        {
            return Err(Box::new(errors::CustomError::new(
                "\nCould not decrypt database. Check master password and try again.",
            )));
        }
    }

    Ok(())
}

pub fn create_new_db(path: &String) -> Result<Connection, Box<dyn std::error::Error>> {
    let create_query = "CREATE TABLE account (\
	 acc_id INTEGER PRIMARY KEY AUTOINCREMENT,\
	 acc_name VARCHAR(100) NOT NULL UNIQUE,\
	 user_name VARCHAR(100) NOT NULL,\
	 password VARCHAR(256) NOT NULL\
	);\
	CREATE TABLE api_key (\
	api_id INTEGER PRIMARY KEY AUTOINCREMENT,\
	api_name VARCHAR(100) NOT NULL,\
	service VARCHAR(100) NOT NULL,\
	user_name VARCHAR(100) NOT NULL,\
	api_key VARCHAR(256) NOT NULL\
	);";

    let master_pass =
        ioutils::get_terminal_input("Enter master password (Make sure to remember it)")?;
    let pragma_query = format!("PRAGMA key = '{}';", &master_pass);
    let dbcon = Connection::open(path)?;

    dbcon.execute_batch(&pragma_query)?;
    dbcon.execute_batch(create_query)?;
    Ok(dbcon)
}

pub fn change_db_password(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let dbcon = Connection::open(path)?;
    decrypt_db(&dbcon)?;

    let master_pass =
        ioutils::get_terminal_input("Enter new master password (Make sure to remember it)")?;
    let pragma_query = format!("PRAGMA rekey = '{}';", &master_pass);

    dbcon.execute_batch(&pragma_query)?;
    Ok(())
}
