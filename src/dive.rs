use std::str::FromStr;

use crate::lib::{
    complete_parsing, default_sub_command, file_to_lines, parse_lines, parse_usize, Command,
};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};
use strum_macros::{EnumString, EnumVariantNames};

pub const DIVE: Command = Command::new(sub_command, "dive", run);

#[derive(Debug)]
struct DiveArgs {
    file: String,
    use_aim: bool,
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
    .arg(
        Arg::with_name("aim")
        .short("a")
        .help("If passed, takes submarine aim into account when determining position.")
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about("Finds the postion for the default input without aim.")
            .version("1.0.0"),
    )
    .subcommand(
        SubCommand::with_name("part2")
            .about("Finds the postion for the default input with aim.")
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let dive_arguments = match arguments.subcommand_name() {
        Some("part1") => DiveArgs {
            file: "day2_dive/input.txt".to_string(),
            use_aim: false,
        },
        Some("part2") => DiveArgs {
            file: "day2_dive/input.txt".to_string(),
            use_aim: true,
        },
        _ => DiveArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            use_aim: arguments.is_present("aim"),
        },
    };

    file_to_lines(&dive_arguments.file)
        .and_then(|lines| parse_lines(lines, complete_parsing(parse_commands)))
        .map(|commands| determine_position(&commands, &dive_arguments.use_aim))
        .map(|(horizontal, depth)| horizontal * depth)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn parse_commands(line: &String) -> IResult<&str, SubmarineCommand> {
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
}

fn determine_position(commands: &Vec<SubmarineCommand>, use_aim: &bool) -> (usize, usize) {
    let position_func = if *use_aim {
        update_position_with_aim
    } else {
        update_position_no_aim
    };
    let (horizontal, depth, _) = commands.into_iter().fold((0, 0, 0), position_func);
    (horizontal, depth)
}

fn update_position_no_aim(
    position: (usize, usize, usize),
    command: &SubmarineCommand,
) -> (usize, usize, usize) {
    match &command.direction {
        Direction::Forward => (position.0 + command.magnitude, position.1, position.2),
        Direction::Down => (position.0, position.1 + command.magnitude, position.2),
        Direction::Up => (position.0, position.1 - command.magnitude, position.2),
    }
}

fn update_position_with_aim(
    position: (usize, usize, usize),
    command: &SubmarineCommand,
) -> (usize, usize, usize) {
    match &command.direction {
        Direction::Forward => (
            position.0 + command.magnitude,
            position.1 + position.2 * command.magnitude,
            position.2,
        ),
        Direction::Down => (position.0, position.1, position.2 + command.magnitude),
        Direction::Up => (position.0, position.1, position.2 - command.magnitude),
    }
}
