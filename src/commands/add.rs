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
            user_name: user_name,
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
            user_name: user_name,
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
    let mut errors_str = String::from("");
    let mut successfull: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line == "" {
            lineno += 1;
            continue;
        }
        let fields: Vec<_> = line.split(",").collect();

        if fields[0] == "login" {
            if fields.len() != 4 {
                errors_str.push_str(&format!("Line {}: Wrong number of fields\n", lineno));
                lineno += 1;
                continue;
            }
            let exists = db::check_account_exists(fields[1], dbcon)?;
            if exists {
                errors_str.push_str(&format!(
                    "Line {}: Account {} already exists\n",
                    lineno, fields[1]
                ));
                lineno += 1;
                continue;
            } else if fields[1] == "master" {
                errors_str.push_str(&format!(
                    "Line {}: Account name cannot be master.\n",
                    lineno
                ));
                lineno += 1;
                continue;
            } else if fields[1] == "" {
                errors_str.push_str(&format!("Line {}: Account name cannot be empty.\n", lineno));
                lineno += 1;
                continue;
            }
            let mut pass: String = fields[3].to_string();
            if pass == "?" {
                pass = passgen::get_random_pass(passlen)?;
            }
            let acc = AccountObj {
                account_name: fields[1].to_string(),
                user_name: fields[2].to_string(),
                password: pass,
            };
            match db::add_account_to_db(&acc, dbcon) {
                Err(e) => errors_str.push_str(&format!("Line {}: {}", lineno, e.to_string())),
                Ok(_) => successfull.push(fields[1].to_string()),
            }
        } else if fields[0] == "api" {
            if fields.len() != 5 {
                errors_str.push_str(&format!("Line {}: Wrong number of fields\n", lineno));
                lineno += 1;
                continue;
            }
            let exists = db::check_apikey_exists(fields[1], dbcon)?;
            if exists {
                errors_str.push_str(&format!(
                    "Line {}: API Key {} already exists\n",
                    lineno, fields[1]
                ));
                lineno += 1;
                continue;
            } else if fields[1] == "master" {
                errors_str.push_str(&format!("Line {}: Api name cannot be master.\n", lineno));
                lineno += 1;
                continue;
            } else if fields[1] == "" {
                errors_str.push_str(&format!("Line {}: Api name cannot be empty.\n", lineno));
                lineno += 1;
                continue;
            }
            let api = APIObj {
                api_name: fields[1].to_string(),
                user_name: fields[2].to_string(),
                description: fields[3].to_string(),
                api_key: fields[4].to_string(),
            };

            match db::add_apikey_to_db(&api, dbcon) {
                Err(e) => errors_str.push_str(&format!("Line {}: {}", lineno, e.to_string())),
                Ok(_) => successfull.push(fields[1].to_string()),
            }
        } else {
            errors_str.push_str(&format!(
                "Line {}: First field should be 'login' or 'api'\n",
                lineno
            ));
        }
        lineno += 1;
    }
    if errors_str != "" {
        println!("Got some errors:\n{}", errors_str);
        println!("Use cman add --help for more details");
    }
    if successfull.len() > 0 {
        println!("\nSuccessfully added:");
        for name in successfull {
            print!("{} ", name);
        }
        println!();
    }
    Ok(())
}
