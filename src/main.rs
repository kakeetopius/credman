use clap::Parser;
use credman::commands::run_command;
use credman::util::*;

fn main() {
    let cli_args = argparser::CmanArgs::parse();

    let result = run_command(&cli_args);

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
