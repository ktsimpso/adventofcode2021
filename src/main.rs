#![feature(const_fn_fn_ptr_basics)]
#![feature(map_first_last)]

mod binary_diagnostic;
mod dive;
mod dumbo_octopus;
mod giant_squid;
mod hydrothermal_venture;
mod lanternfish;
mod lib;
mod passage_pathing;
mod seven_segment;
mod smoke_basin;
mod sonar_sweep;
mod syntax_scoring;
mod transparent_origami;
mod whale_treachery;
mod extended_polymerization;
mod chiton;

use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings};
#[macro_use]
extern crate lazy_static;
use lib::Command;
use simple_error::SimpleError;
use std::collections::HashMap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref COMMANDS: Vec<Box<dyn Command>> = vec![
        Box::new(sonar_sweep::SONAR_SWEEP),
        Box::new(dive::DIVE),
        Box::new(binary_diagnostic::BINARY_DIAGNOSTIC),
        Box::new(giant_squid::GIANT_SQUID),
        Box::new(hydrothermal_venture::HYDROTHERMAL_VENTURE),
        Box::new(lanternfish::LANTERNFISH),
        Box::new(whale_treachery::WHALE_TREACHERY),
        Box::new(seven_segment::SEVEN_SEGMENT),
        Box::new(smoke_basin::SMOKE_BASIN),
        Box::new(syntax_scoring::SYNTAX_SCORING),
        Box::new(dumbo_octopus::DUMBO_OCTOPUS),
        Box::new(passage_pathing::PASSAGE_PATHING),
        Box::new(transparent_origami::TRANSPARENT_ORIGAMI),
        Box::new(extended_polymerization::EXTENDED_POLYMERIZATION),
        Box::new(chiton::CHITON),
    ];
}

fn main() -> Result<(), Error> {
    let app = App::new("Advent of code 2021")
        .version(VERSION)
        .about("Run the advent of code problems from this main program")
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let matches = COMMANDS
        .iter()
        .fold(app, |app, command| app.subcommand(command.sub_command()))
        .get_matches();

    let sub_commands: HashMap<&str, &Box<dyn Command>> = COMMANDS
        .iter()
        .map(|command| (command.name(), command))
        .collect();

    if let (command_name, Some(args)) = matches.subcommand() {
        sub_commands
            .get(command_name)
            .ok_or_else::<Error, _>(|| SimpleError::new("No valid subcommand found").into())
            .and_then(|command| {
                println!("=============Running {:}=============", command.name());
                let file = match args.subcommand_name() {
                    Some("part1") => format!("{}/input.txt", command.folder_name()),
                    Some("part2") => format!("{}/input.txt", command.folder_name()),
                    _ => format!(
                        "{}/{}",
                        command.folder_name(),
                        value_t_or_exit!(args.value_of("file"), String)
                    ),
                };

                command.run(args, &file)
            })
            .map(|result| {
                println!("{:#?}", result);
            })
            .map(|_| ())
    } else {
        Err(SimpleError::new("No arguments found").into())
    }
}
