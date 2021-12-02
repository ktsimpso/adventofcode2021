use std::str::FromStr;

use crate::lib::{default_sub_command, file_to_lines, parse_lines, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};
use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{map, map_res},
    sequence::separated_pair,
};
use simple_error::SimpleError;
use strum_macros::{EnumString, EnumVariantNames};

pub const DIVE: Command = Command::new(sub_command, "dive", run);

#[derive(Debug)]
struct DiveArgs {
    file: String,
}

#[derive(Debug, EnumString, EnumVariantNames, Clone)]
#[strum(serialize_all = "kebab_case")]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct SubmarineCommand {
    direction: Direction,
    magnitude: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &DIVE,
        "Finds the final position of the sub starting at 0,0 then returns the multiple of the tuple.",
        "Path to the input file. Each line should contain a direction followed by a number.",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about("Finds the postion for the default input.")
            .version("1.0.0"),
    )
    /*.subcommand(
        SubCommand::with_name("part2")
            .about("I will find out")
            .version("1.0.0"),
    )*/
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let dive_arguments = match arguments.subcommand_name() {
        Some("part1") => DiveArgs {
            file: "day2_dive/input.txt".to_string(),
        },
        Some("part2") => DiveArgs {
            file: "day2_dive/input.txt".to_string(),
        },
        _ => DiveArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    file_to_lines(&dive_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_commands))
        .map(|commands| determine_position(&commands))
        .map(|(horizontal, depth)| horizontal * depth)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn parse_commands(line: &String) -> Result<SubmarineCommand, Error> {
    map(
        separated_pair(
            map_res(complete::alpha1, Direction::from_str),
            tag(" "),
            parse_usize,
        ),
        |(direction, magnitude)| SubmarineCommand {
            direction,
            magnitude,
        },
    )(line)
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error").into())
    .map(|(_, command)| command)
}

fn determine_position(commands: &Vec<SubmarineCommand>) -> (usize, usize) {
    commands.into_iter().fold((0, 0), update_position)
}

fn update_position(position: (usize, usize), command: &SubmarineCommand) -> (usize, usize) {
    match &command.direction {
        Direction::Forward => (position.0 + command.magnitude, position.1),
        Direction::Down => (position.0, position.1 + command.magnitude),
        Direction::Up => (position.0, position.1 - command.magnitude),
    }
}
