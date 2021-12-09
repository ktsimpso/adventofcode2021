use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::map_parser,
    multi::{many1, separated_list0},
    IResult,
};

pub const SMOKE_BASIN: Problem<SmokeBasinArgs, Vec<Vec<usize>>> = Problem::new(
    sub_command,
    "smoke-basin",
    "day9_smoke_basin",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct SmokeBasinArgs {}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SMOKE_BASIN,
        "Finds low points in lava tube smoke stacks then calculates values",
        "Path to the input file. Input should be integers and newlines with equal sizes.",
        "Searches the default input for the total risk level.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SmokeBasinArgs {
    match arguments.subcommand_name() {
        Some("part1") => SmokeBasinArgs {},
        Some("part2") => SmokeBasinArgs {},
        _ => SmokeBasinArgs {},
    }
}

fn run(arguments: SmokeBasinArgs, smoke_points: Vec<Vec<usize>>) -> CommandResult {
    let mut risk_level = 0usize;
    let column_length = smoke_points.len();
    for i in 0..column_length {
        let row = smoke_points.get(i).unwrap();
        let row_length = row.len();
        for j in 0..row_length {
            let current = row.get(j).unwrap();
            let adjacents = get_adjacent_indicies((&i, &j), &column_length, &row_length);
            let low_point = adjacents
                .iter()
                .map(|(x, y)| smoke_points.get(*x).unwrap().get(*y).unwrap())
                .all(|value| current < value);

            if low_point {
                risk_level += current + 1;
            }
        }
    }

    risk_level.into()
}

fn get_adjacent_indicies(
    point: (&usize, &usize),
    column_length: &usize,
    row_length: &usize,
) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let mut adjacents = Vec::new();
    if *y > 0usize {
        adjacents.push((*x, y - 1));
    }

    if *y < (*row_length - 1usize) {
        adjacents.push((*x, y + 1));
    }

    if *x > 0usize {
        adjacents.push((x - 1, *y));
    }

    if *x < (*column_length - 1usize) {
        adjacents.push((x + 1, *y));
    }

    adjacents
}

fn parse_data(input: &String) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    many1(map_parser(take(1usize), parse_usize))(input)
}
