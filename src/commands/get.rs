use crate::commands::*;

pub fn get_account_from_user(dbcon: &Connection) -> core::result::Result<Secret, CMError> {
    let all_accounts = db::get_all_accounts_from_db(dbcon)?;
    if all_accounts.len() < 1 {
        return Err(CustomError::new(
            "No accounts added yet. Use cman add <account_name> to add your first account. See cman add --help for more details.",
        ).into());
    }
    get_terminal_input_with_suggestions("Enter the account name", "", all_accounts)
}

pub fn get_api_from_user(dbcon: &Connection) -> core::result::Result<Secret, CMError> {
    let all_api_keys = db::get_all_apikeys_from_db(dbcon)?;
    if all_api_keys.len() < 1 {
        return Err(CustomError::new(
            "No api keys added yet. Use cman add <api_name> to add your first api key. See cman add --help for more details.",
        ).into());
    }
    get_terminal_input_with_suggestions("Enter the api key name", "", all_api_keys)
}

pub fn run_get(args: &GetArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let secret = match &args.secret {
        Some(s) => match sec_type {
            SecretType::Login => db::get_account_from_db(&s, &dbcon)?,
            SecretType::Api => db::get_apikey_from_db(&s, &dbcon)?,
        },
        None => match sec_type {
            SecretType::Login => get_account_from_user(dbcon)?,
            SecretType::Api => get_api_from_user(dbcon)?,
        },
    };

    if let Some(fieldtype) = args.field {
        if args.json {
            secret.print_field_json(fieldtype);
        } else {
            secret.print_field(fieldtype);
        }
        return Ok(());
    }

    if args.json {
        secret.print_json();
        return Ok(());
    }
    secret.print();
    Ok(())
}
