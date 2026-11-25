use anyhow::anyhow;
use rusqlite::Connection;

use crate::{
    objects::{KeyDetails, LoginDetails, Secret},
    util::argparser::FieldType,
};

pub fn check_account_exists(
    account_name: &str,
    dbcon: &Connection,
) -> Result<bool, rusqlite::Error> {
    let query = "SELECT EXISTS(SELECT 1 FROM account WHERE acc_name = ?1);";

    let result = dbcon.query_row(query, (account_name,), |row| row.get::<_, i32>(0))?;
    Ok(result != 0)
}

pub fn check_key_exists(key_name: &str, dbcon: &Connection) -> Result<bool, rusqlite::Error> {
    let query = "SELECT EXISTS(SELECT 1 FROM api_keys WHERE api_name = ?1);";

    let result = dbcon.query_row(query, (key_name,), |row| row.get::<_, i32>(0))?;
    Ok(result != 0)
}

pub fn add_account_to_db(
    account: &LoginDetails,
    dbcon: &Connection,
) -> Result<usize, rusqlite::Error> {
    let query = "INSERT INTO account(acc_name, user_name, password) VALUES (?1, ?2, ?3);";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows =
        stmt.execute([&account.account_name, &account.user_name, &account.password])?;
    Ok(affected_rows)
}

pub fn add_key_to_db(key: &KeyDetails, dbcon: &Connection) -> Result<usize, rusqlite::Error> {
    let query =
        "INSERT INTO api_keys(api_name, description, user_name, api_key) VALUES (?1, ?2, ?3, ?4);";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows = stmt.execute([
        &key.key_name,
        &key.description,
        &key.user_name,
        &key.key_value,
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

pub fn delete_key_from_db(key_name: &str, dbcon: &Connection) -> Result<usize, rusqlite::Error> {
    let query = "DELETE FROM api_keys WHERE api_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let affected_rows = stmt.execute([key_name])?;
    Ok(affected_rows)
}

pub fn get_account_from_db(account_name: &str, dbcon: &Connection) -> anyhow::Result<Secret> {
    let query = "SELECT acc_name, user_name, password, created_at, password_last_changed FROM account WHERE acc_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let mut results = stmt.query([account_name])?;
    let result = results.next()?;

    if let Some(row) = result {
        Ok(LoginDetails {
            account_name: row.get(0)?,
            user_name: row.get(1)?,
            password: row.get(2)?,
            created_at: row.get(3)?,
            password_last_changed: row.get(4)?,
        }
        .into())
    } else {
        Err(anyhow!(format!("Account {} not found", account_name)))
    }
}

pub fn get_key_from_db(key_name: &str, dbcon: &Connection) -> anyhow::Result<Secret> {
    let query = "SELECT api_name, description, user_name, api_key, created_at, api_key_last_changed FROM api_keys WHERE api_name = ?1;";
    let mut stmt = dbcon.prepare(query)?;
    let mut results = stmt.query([key_name])?;
    let result = results.next()?;

    if let Some(row) = result {
        Ok(KeyDetails {
            key_name: row.get(0)?,
            description: row.get(1)?,
            user_name: row.get(2)?,
            key_value: row.get(3)?,
            created_at: row.get(4)?,
            key_last_changed: row.get(5)?,
        }
        .into())
    } else {
        Err(anyhow!(format!("Key {} not found", key_name)))
    }
}

pub fn get_all_accounts_from_db(dbcon: &Connection) -> Result<Vec<Secret>, rusqlite::Error> {
    let query =
        "SELECT acc_name, user_name, password, created_at, password_last_changed FROM account;";
    let mut stmt = dbcon.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok(LoginDetails {
            account_name: row.get(0)?,
            user_name: row.get(1)?,
            password: row.get(2)?,
            created_at: row.get(3)?,
            password_last_changed: row.get(4)?,
        })
    })?;

    let mut results: Vec<Secret> = Vec::new();

    for row in rows.flatten() {
        results.push(row.into())
    }
    Ok(results)
}

pub fn get_all_keys_from_db(dbcon: &Connection) -> Result<Vec<Secret>, rusqlite::Error> {
    let query = "SELECT api_name, description, user_name, api_key, created_at, api_key_last_changed FROM api_keys;";
    let mut stmt = dbcon.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok(KeyDetails {
            key_name: row.get(0)?,
            description: row.get(1)?,
            user_name: row.get(2)?,
            key_value: row.get(3)?,
            created_at: row.get(4)?,
            key_last_changed: row.get(5)?,
        })
    })?;

    let mut results: Vec<Secret> = Vec::new();
    for result in rows.into_iter().flatten() {
        results.push(result.into());
    }
    Ok(results)
}

pub fn change_db_account_field(
    account_name: &str,
    field: FieldType,
    new_value: &str,
    dbcon: &Connection,
) -> anyhow::Result<usize> {
    let field_to_change = match field {
        FieldType::User => "user_name",
        FieldType::Secname => "acc_name",
        FieldType::Pass => "password",
        _ => {
            return Err(anyhow!("The given field is invalid for a login credential"));
        }
    };

    let query = format!(
        "UPDATE account SET {} = ?1 WHERE acc_name = ?2;",
        field_to_change
    );
    let mut stmt = dbcon.prepare(&query)?;
    let affected_rows = stmt.execute([new_value, account_name])?;
    Ok(affected_rows)
}

pub fn change_db_key_field(
    key_name: &str,
    field: FieldType,
    new_value: &str,
    dbcon: &Connection,
) -> anyhow::Result<usize> {
    let field_to_change = match field {
        FieldType::User => "user_name",
        FieldType::Secname => "api_name",
        FieldType::Desc => "description",
        FieldType::Key => "api_key",
        _ => {
            return Err(anyhow!("The given field is invalid for a key."));
        }
    };

    let query = format!(
        "UPDATE api_keys SET {} = ?1 WHERE api_name = ?2;",
        field_to_change
    );
    let mut stmt = dbcon.prepare(&query)?;
    let affected_rows = stmt.execute([new_value, key_name])?;
    Ok(affected_rows)
}
