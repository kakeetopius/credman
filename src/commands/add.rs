use crate::commands::*;

pub fn run_add(args: &AddArgs, dbcon: &Connection) -> Result {
    let sec_type = args.secret_type.unwrap_or(SecretType::Login);
    let sec_name = &args.secret;
    if sec_name == "master" {
        return Err(CustomError::new(
            "Cannot use the name \"master\" because it is reserved for the master password",
        )
        .into());
    } else if args.batch {
        return add_secrets_from_batch(sec_name, args.passlen, dbcon);
    }

    match sec_type {
        SecretType::Login => add_new_acc(sec_name, args.passlen, args.no_auto, dbcon)?,
        SecretType::Api => add_new_api(sec_name, dbcon)?,
    };
    println!("Added Successfully");
    Ok(())
}

fn add_new_acc(name: &str, passlen: Option<usize>, noautopass: bool, dbcon: &Connection) -> Result {
    let exists = db::check_account_exists(name, dbcon)?;
    if exists {
        return Err(CustomError::new(&format!("Account {} already exists", name)).into());
    }
    let user_name = get_terminal_input("Enter username for the account", false, false)?;

    let pass = if noautopass {
        get_terminal_input("Enter Password", true, true)?
    } else {
        passgen::get_random_pass(passlen)?
    };

    db::add_account_to_db(
        &AccountObj {
            account_name: name.to_string(),
            user_name,
            password: pass,
        },
        dbcon,
    )?;
    Ok(())
}

fn add_new_api(name: &str, dbcon: &Connection) -> Result {
    let exists = db::check_apikey_exists(name, dbcon)?;
    if exists {
        return Err(CustomError::new(&format!("API Key {} already exists", name)).into());
    }
    let user_name = get_terminal_input(
        "Enter username for the account associated with API Key (if any)",
        false,
        false,
    )?;
    let desc = get_terminal_input("Enter a short description for the API key", false, false)?;
    let apikey = get_terminal_input("Enter API Key", false, false)?;

    db::add_apikey_to_db(
        &APIObj {
            api_name: name.to_string(),
            description: desc,
            user_name,
            api_key: apikey,
        },
        dbcon,
    )?;
    Ok(())
}

fn add_secrets_from_batch(batch_file: &str, passlen: Option<usize>, dbcon: &Connection) -> Result {
    let file = File::open(batch_file)?;
    let reader = BufReader::new(file);
    let mut lineno = 1;
    let mut errors: Vec<CMError> = Vec::new();
    let mut successfull: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            lineno += 1;
            continue;
        }
        let fields: Vec<_> = line.split(",").collect();

        if fields[0] == "login" {
            let result = add_acc_from_file_line(dbcon, &fields, lineno, passlen);
            match result {
                Ok(name) => successfull.push(name),
                Err(e) => errors.push(e),
            }
        } else if fields[0] == "api" {
            let result = add_api_from_file_line(dbcon, &fields, lineno);
            match result {
                Ok(name) => successfull.push(name),
                Err(e) => errors.push(e),
            }
        } else {
            errors.push(
                CustomError::new(&format!(
                    "Line {}: First field should be 'login' or 'api'\n",
                    lineno
                ))
                .into(),
            );
        }
        lineno += 1;
    }

    if !errors.is_empty() {
        println!("Got some errors:\n");
        errors.iter().for_each(|e| println!("{}", e));
        println!("\nUse cman add --help for more details");
    }
    if !successfull.is_empty() {
        println!("\nSuccessfully added:");
        successfull.iter().for_each(|n| println!("{}", n));
    }
    Ok(())
}

fn add_acc_from_file_line(
    dbcon: &Connection,
    fields: &[&str],
    lineno: i32,
    passlen: Option<usize>,
) -> std::result::Result<String, CMError> {
    if fields.len() != 4 {
        return Err(CustomError::new(&format!("Line {}: Wrong number of fields", lineno)).into());
    }
    let (account_name, user_name) = (fields[1], fields[2]);
    let pass = if fields[3] == "?" {
        passgen::get_random_pass(passlen)?
    } else {
        fields[3].to_string()
    };

    let exists = db::check_account_exists(account_name, dbcon)?;
    if exists {
        return Err(CustomError::new(&format!(
            "Line {}: Account {} already exists",
            lineno, account_name
        ))
        .into());
    } else if account_name == "master" {
        return Err(
            CustomError::new(&format!("Line {}: Account name cannot be master.", lineno)).into(),
        );
    } else if account_name.is_empty() {
        return Err(
            CustomError::new(&format!("Line {}: Account name cannot be empty.", lineno)).into(),
        );
    } else if pass.is_empty() {
        return Err(
            CustomError::new(&format!("Line {}: No password provided. Use ? as the password if password generation for the account is required.", lineno)).into()
        );
    }

    let acc = AccountObj {
        account_name: account_name.to_string(),
        user_name: user_name.to_string(),
        password: pass,
    };

    db::add_account_to_db(&acc, dbcon)?;

    Ok(acc.account_name)
}

fn add_api_from_file_line(
    dbcon: &Connection,
    fields: &[&str],
    lineno: i32,
) -> std::result::Result<String, CMError> {
    if fields.len() != 5 {
        return Err(CustomError::new(&format!("Line {}: Wrong number of fields", lineno)).into());
    }

    let (api_name, user_name, description, api_key) = (fields[1], fields[2], fields[3], fields[4]);

    let exists = db::check_apikey_exists(fields[1], dbcon)?;
    if exists {
        return Err(CustomError::new(&format!(
            "Line {}: API Key {} already exists",
            lineno, api_name
        ))
        .into());
    } else if api_name == "master" {
        return Err(
            CustomError::new(&format!("Line {}: Api name cannot be master.", lineno)).into(),
        );
    } else if api_name.is_empty() {
        return Err(
            CustomError::new(&format!("Line {}: Api name cannot be empty.", lineno)).into(),
        );
    } else if api_key.is_empty() {
        return Err(CustomError::new(&format!("Line {}: No Api Key provided", lineno)).into());
    }

    let api = APIObj {
        api_name: api_name.to_string(),
        user_name: user_name.to_string(),
        description: description.to_string(),
        api_key: api_key.to_string(),
    };

    db::add_apikey_to_db(&api, dbcon)?;

    Ok(api.api_name)
}
