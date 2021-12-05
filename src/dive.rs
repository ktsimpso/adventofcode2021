use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;
use strum_macros::{EnumString, EnumVariantNames};

pub const DIVE: Problem<DiveArgs, Vec<SubmarineCommand>> = Problem::new(
    sub_command,
    "dive",
    "day2_dive",
    parse_arguments,
    parse_commands,
    run,
);

#[derive(Debug)]
pub struct DiveArgs {
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
pub struct SubmarineCommand {
    direction: Direction,
    magnitude: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &DIVE,
        "Finds the final position of the sub starting at 0,0 then returns the multiple of the tuple.",
        "Path to the input file. Each line should contain a direction followed by a number.",
        "Finds the postion for the default input without aim.",
        "Finds the postion for the default input with aim.",
    )
    .arg(
        Arg::with_name("aim")
        .short("a")
        .help("If passed, takes submarine aim into account when determining position.")
    )
}

fn parse_arguments(arguments: &ArgMatches) -> DiveArgs {
    match arguments.subcommand_name() {
        Some("part1") => DiveArgs { use_aim: false },
        Some("part2") => DiveArgs { use_aim: true },
        _ => DiveArgs {
            use_aim: arguments.is_present("aim"),
        },
    }
}

fn run(arguments: DiveArgs, commands: Vec<SubmarineCommand>) -> CommandResult {
    let (horizontal, depth) = determine_position(&commands, &arguments.use_aim);
    (horizontal * depth).into()
}

fn parse_commands(line: &String) -> IResult<&str, Vec<SubmarineCommand>> {
    separated_list0(
        newline,
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
        ),
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
