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
    let accounts = match &args.secret {
        Some(accounts) => accounts.clone(),
        None => {
            if args.multiple {
                let selections: Vec<String> = get::get_multiple_accounts_from_user(dbcon)?
                    .iter()
                    .map(|secret| secret.get_name())
                    .collect();
                selections
            } else {
                let acc_obj = get_account_from_user(dbcon)?;
                if let Secret::Account(acc) = acc_obj {
                    vec![acc.account_name.clone()]
                } else {
                    vec!["".to_string()]
                }
            }
        }
    };

    let mut error_str = String::new();
    let mut successfull: Vec<String> = Vec::new();
    for account in accounts {
        let exists = db::check_account_exists(&account, dbcon)?;
        if !exists {
            error_str.push_str(&format!("Account {} does not exist\n", account));
            continue;
        }
        let opt = get_user_confirmation(&format!(
            "Are you sure you want to delete {} (yes/no)",
            account
        ))?;
        if !opt {
            continue;
        }
        db::delete_account_from_db(&account, dbcon)?;
        successfull.push(account)
    }

    if !successfull.is_empty() {
        println!("\nSuccessfully deleted:");
        for name in successfull {
            print!("{} ", name);
        }
        println!();
    }
    if !error_str.is_empty() {
        return Err(CustomError::new(&error_str).into());
    }
    Ok(())
}

fn delete_api(args: &DeleteArgs, dbcon: &Connection) -> Result {
    let apikeys = match &args.secret {
        Some(apikeys) => apikeys.clone(),
        None => {
            if args.multiple {
                get::get_multiple_apikeys_from_user(dbcon)?
                    .iter()
                    .map(|secret| secret.get_name())
                    .collect()
            } else {
                let api_obj = get_api_from_user(dbcon)?;
                if let Secret::API(api) = api_obj {
                    vec![api.api_name.clone()]
                } else {
                    vec!["".to_string()]
                }
            }
        }
    };
    let mut error_str = String::new();
    let mut successfull: Vec<String> = Vec::new();
    for apikey in apikeys {
        let exists = db::check_apikey_exists(&apikey, dbcon)?;
        if !exists {
            error_str.push_str(&format!("API Key {} does not exist\n", apikey));
            continue;
        }
        let opt = get_user_confirmation(&format!(
            "Are you sure you want to delete {} (yes/no)",
            apikey
        ))?;
        if !opt {
            continue;
        }
        db::delete_apikey_from_db(&apikey, dbcon)?;
        successfull.push(apikey)
    }

    if !successfull.is_empty() {
        println!("\nSuccessfully deleted:");
        for name in successfull {
            print!("{} ", name);
        }
        println!();
    }
    if !error_str.is_empty() {
        return Err(CustomError::new(&error_str).into());
    }
    Ok(())
}
