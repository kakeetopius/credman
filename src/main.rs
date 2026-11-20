use clap::Parser;
use credman::{
    commands::run_command,
    util::{argparser, ioutils::set_terminal_settings},
};

fn main() -> anyhow::Result<()> {
    let cli_args = argparser::CmanArgs::parse();
    set_terminal_settings(&cli_args);
    run_command(cli_args)
}
