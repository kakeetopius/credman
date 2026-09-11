use std::{fs, io, io::Write};

use crate::{
    commands::*,
    util::argparser::{ExportArgs, ExportFormats},
};

pub fn run_export(args: &ExportArgs, dbcon: &Connection) -> Result {
    let secrets = if args.all {
        let mut results = Vec::new();
        results.extend(db::get_all_accounts_from_db(dbcon)?);
        results.extend(db::get_all_apikeys_from_db(dbcon)?);

        results
    } else {
        let secret_type = args.secret_type.unwrap_or(SecretType::Login);
        match secret_type {
            SecretType::Login => db::get_all_accounts_from_db(dbcon)?,
            SecretType::Api => db::get_all_apikeys_from_db(dbcon)?,
        }
    };

    let mut out: Box<dyn Write> = if let Some(file) = &args.output_file {
        let f = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file)?;
        Box::new(f)
    } else {
        Box::new(io::stdout())
    };

    match args.format {
        ExportFormats::Json => export_json(secrets, &mut *out, args.pretty),
        ExportFormats::List => export_txt(secrets, &mut *out),
    }
}

fn export_txt<W>(secrets: Vec<Secret>, out: &mut W) -> Result
where
    W: Write + ?Sized,
{
    let ips_strings: Vec<String> = secrets
        .iter()
        .map(|i| i.to_comma_separated_string())
        .collect();

    let ips_string = ips_strings.join("\n");

    std::io::copy(&mut ips_string.as_bytes(), out)?;

    Ok(())
}

fn export_json<W>(secrets: Vec<Secret>, out: &mut W, pretty: bool) -> Result
where
    W: Write + ?Sized,
{
    if pretty {
        serde_json::to_writer_pretty(out, &secrets)?;
    } else {
        serde_json::to_writer(out, &secrets)?;
    }

    Ok(())
}
