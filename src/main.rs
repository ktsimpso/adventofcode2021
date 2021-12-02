#![feature(const_fn_fn_ptr_basics)]

mod lib;
mod sonar_sweep;
mod dive;

use anyhow::Error;
use clap::{App, AppSettings};
use lib::Command;
use simple_error::SimpleError;
use std::collections::HashMap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const COMMANDS: &'static [Command] = &[
    sonar_sweep::SONAR_SWEEP,
    dive::DIVE
];

fn main() -> Result<(), Error> {
    let app = App::new("Advent of code 2021")
        .version(VERSION)
        .about("Run the advent of code problems from this main program")
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let matches = COMMANDS
        .iter()
        .fold(app, |app, command| app.subcommand(command.sub_command()))
        .get_matches();

    let sub_commands: HashMap<&str, &Command> = COMMANDS
        .iter()
        .map(|command| (command.name(), command))
        .collect();

    if let (command_name, Some(args)) = matches.subcommand() {
        sub_commands
            .get(command_name)
            .ok_or_else::<Error, _>(|| SimpleError::new("No valid subcommand found").into())
            .and_then(|command| {
                println!("=============Running {:}=============", command.name());
                command.run(args)
            })
    } else {
        Err(SimpleError::new("No arguments found").into())
    }
}
