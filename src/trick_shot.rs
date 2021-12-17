use crate::lib::{default_sub_command, parse_isize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

pub const TRICK_SHOT: Problem<TrickShotArgs, Target> = Problem::new(
    sub_command,
    "trick-shot",
    "day17_trick_shot",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct TrickShotArgs {}

#[derive(Debug)]
pub struct Target {
    lower_x: isize,
    upper_x: isize,
    lower_y: isize,
    upper_y: isize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &TRICK_SHOT,
        "Calculates valid trajectories for a target.",
        "Path to the input file. Input should be the target area.",
        "Finds the maximum height that can be acheived while still hitting the target.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> TrickShotArgs {
    match arguments.subcommand_name() {
        Some("part1") => TrickShotArgs {},
        Some("part2") => TrickShotArgs {},
        _ => TrickShotArgs {},
    }
}

fn run(arguments: TrickShotArgs, target: Target) -> CommandResult {
    find_max_possible_height(&target).into()
}

fn find_max_possible_height(target: &Target) -> isize {
    let y = target.lower_y.abs() - 1;
    max_y(&y)
}

fn max_y(y: &isize) -> isize {
    y_at_n(&y, &*y)
}

fn y_at_n(y: &isize, n: &isize) -> isize {
    ((2isize * y + 1isize) * n - (n * n)) / 2
}

fn parse_data(input: &String) -> IResult<&str, Target> {
    map(
        tuple((
            preceded(tag("target area: x="), parse_isize),
            preceded(tag(".."), parse_isize),
            preceded(tag(", y="), parse_isize),
            preceded(tag(".."), parse_isize),
        )),
        |(lower_x, upper_x, lower_y, upper_y)| Target {
            lower_x: lower_x,
            upper_x: upper_x,
            lower_y: lower_y,
            upper_y: upper_y,
        },
    )(input)
}
