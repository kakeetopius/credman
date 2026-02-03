use crate::objects::{APIObj, AccountObj, Secret};
use crate::util::argparser::FieldType;
use crate::util::errors::{CMError, CustomError};

use rusqlite::Connection;

pub fn check_account_exists(
    account_name: &str,
    dbcon: &Connection,
) -> Result<bool, rusqlite::Error> {
    let query = "SELECT EXISTS(SELECT 1 FROM account WHERE acc_name = ?1);";

    let result = dbcon.query_row(query, (account_name,), |row| row.get::<_, i32>(0))?;
    if result == 0 {
        return Ok(false);
    }

    Ok(true)
}

pub fn check_apikey_exists(apikey_name: &str, dbcon: &Connection) -> Result<bool, rusqlite::Error> {
    let query = "SELECT EXISTS(SELECT 1 FROM api_keys WHERE api_name = ?1);";

    let result = dbcon.query_row(query, (apikey_name,), |row| row.get::<_, i32>(0))?;
    if result == 0 {
        return Ok(false);
    }

    Ok(true)
}

pub fn add_account_to_db(
    account: &AccountObj,
    dbcon: &Connection,
) -> Result<usize, rusqlite::Error> {
    let query = "INSERT INTO account(acc_name, user_name, password) VALUES (?1, ?2, ?3);";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows =
        stmt.execute([&account.account_name, &account.user_name, &account.password])?;
    Ok(affected_rows)
}

pub fn add_apikey_to_db(api: &APIObj, dbcon: &Connection) -> Result<usize, rusqlite::Error> {
    let query =
        "INSERT INTO api_keys(api_name, description, user_name, api_key) VALUES (?1, ?2, ?3, ?4);";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows = stmt.execute([
        &api.api_name,
        &api.description,
        &api.user_name,
        &api.api_key,
    ])?;
    Ok(affected_rows)
}

pub fn delete_account_from_db(
    account_name: &str,
    dbcon: &Connection,
) -> Result<usize, rusqlite::Error> {
    let query = "DELETE FROM account WHERE acc_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows = stmt.execute([account_name])?;
    Ok(affected_rows)
}

pub fn delete_apikey_from_db(
    apikey_name: &str,
    dbcon: &Connection,
) -> Result<usize, rusqlite::Error> {
    let query = "DELETE FROM api_keys WHERE api_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows = stmt.execute([apikey_name])?;
    Ok(affected_rows)
}

pub fn get_account_from_db(account_name: &str, dbcon: &Connection) -> Result<Secret, CMError> {
    let query = "SELECT acc_name, user_name, password FROM account WHERE acc_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let mut results = stmt.query([account_name])?;
    let result = results.next()?;

    if let Some(row) = result {
        Ok(AccountObj {
            account_name: row.get(0)?,
            user_name: row.get(1)?,
            password: row.get(2)?,
        }
        .into())
    } else {
        Err(CustomError::new(&format!("Account {} not found", account_name)).into())
    }
}

pub fn get_apikey_from_db(apikey_name: &str, dbcon: &Connection) -> Result<Secret, CMError> {
    let query =
        "SELECT api_name, description, user_name, api_key FROM api_keys WHERE api_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let mut results = stmt.query([apikey_name])?;
    let result = results.next()?;

    if let Some(row) = result {
        Ok(APIObj {
            api_name: row.get(0)?,
            description: row.get(1)?,
            user_name: row.get(2)?,
            api_key: row.get(3)?,
        }
        .into())
    } else {
        Err(CustomError::new(&format!("API Key {} not found", apikey_name)).into())
    }
}

pub fn get_all_accounts_from_db(dbcon: &Connection) -> Result<Vec<Secret>, rusqlite::Error> {
    let query = "SELECT acc_name, user_name, password FROM account;";
    let mut stmt = dbcon.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok(AccountObj {
            account_name: row.get(0)?,
            user_name: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    let mut results: Vec<Secret> = Vec::new();
    for row in rows {
        if let Ok(result) = row {
            results.push(result.into());
        }
    }
    Ok(results)
}

pub fn get_all_apikeys_from_db(dbcon: &Connection) -> Result<Vec<Secret>, rusqlite::Error> {
    let query = "SELECT api_name, description, user_name, api_key FROM api_keys;";
    let mut stmt = dbcon.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok(APIObj {
            api_name: row.get(0)?,
            description: row.get(1)?,
            user_name: row.get(2)?,
            api_key: row.get(3)?,
        })
    })?;

    let mut results: Vec<Secret> = Vec::new();
    for row in rows {
        if let Ok(result) = row {
            results.push(result.into());
        }
    }
    Ok(results)
}

pub fn change_db_account_field(
    account_name: &str,
    field: FieldType,
    new_value: &str,
    dbcon: &Connection,
) -> Result<usize, CMError> {
    let field_to_change = match field {
        FieldType::User => "user_name",
        FieldType::Secname => "acc_name",
        FieldType::Pass => "password",
        _ => {
            return Err(
                CustomError::new("The given field is invalid for a login credential").into(),
            );
        }
    };

    let query = format! { "UPDATE account SET {} = ?1 WHERE acc_name = ?2;", field_to_change};
    let mut stmt = dbcon.prepare(&query)?;
    let affected_rows = stmt.execute([new_value, account_name])?;
    Ok(affected_rows)
}

pub fn change_db_apikey_field(
    api_name: &str,
    field: FieldType,
    new_value: &str,
    dbcon: &Connection,
) -> Result<usize, CMError> {
    let field_to_change = match field {
        FieldType::User => "user_name",
        FieldType::Secname => "api_name",
        FieldType::Desc => "description",
        FieldType::Key => "api_key",
        _ => {
            return Err(CustomError::new("The given field is invalid for an api key.").into());
        }
    };

    let query = format!(
        "UPDATE api_keys SET {} = ?1 WHERE api_name = ?2;",
        field_to_change
    );
    let mut stmt = dbcon.prepare(&query)?;
    let affected_rows = stmt.execute([new_value, api_name])?;
    Ok(affected_rows)
}
