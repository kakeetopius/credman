use clap::Parser;
use credman::{
    commands::run_command,
    util::{argparser, ioutils::set_terminal_settings},
};

fn main() {
    let cli_args = argparser::CmanArgs::parse();

    set_terminal_settings(&cli_args);
    let result = run_command(&cli_args);

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
