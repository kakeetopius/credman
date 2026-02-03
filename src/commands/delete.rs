use crate::commands::*;

pub fn run_delete(args: &DeleteArgs, dbcon: &Connection) -> Result {
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
