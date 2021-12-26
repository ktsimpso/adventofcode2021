use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::value,
    multi::{many1, separated_list0},
    IResult,
};

pub const SEA_CUCUMBER: Problem<SeaCucumberArgs, Vec<Vec<SeaCucumber>>> = Problem::new(
    sub_command,
    "sea-cucumber",
    "day25_sea_cucumber",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct SeaCucumberArgs {}

#[derive(Debug, Copy, Clone)]
pub enum SeaCucumber {
    Right,
    Down,
    None,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SEA_CUCUMBER,
        "Simulates sea cucumber movements and finds the steady state.",
        "Path to the input file. Input should be the initial state of the sea cucumbers.",
        "Returns the number of steps to reach steady state for the default input.",
        "The same as part 1!",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SeaCucumberArgs {
    match arguments.subcommand_name() {
        Some("part1") => SeaCucumberArgs {},
        Some("part2") => SeaCucumberArgs {},
        _ => SeaCucumberArgs {},
    }
}

fn run(_arguments: SeaCucumberArgs, mut sea_cucumbers: Vec<Vec<SeaCucumber>>) -> CommandResult {
    let mut event_count = 0usize;

    loop {
        event_count += 1;
        let count = run_step(&mut sea_cucumbers);
        if count == 0 {
            break;
        }
    }

    event_count.into()
}

fn run_step(sea_cucumbers: &mut Vec<Vec<SeaCucumber>>) -> usize {
    let mut movements = Vec::new();
    for i in 0..sea_cucumbers.len() {
        let column = sea_cucumbers.get(i).expect("Bounds checked");
        for j in 0..column.len() {
            let cucumber = column.get(j).expect("Bounds checked");
            match cucumber {
                SeaCucumber::Right => {
                    let next = if (j + 1) == column.len() {
                        0usize
                    } else {
                        j + 1
                    };
                    let next_space = column.get(next).expect("Bounds Checked");
                    match next_space {
                        SeaCucumber::None => {
                            movements.push((SeaCucumber::Right, (i, j), (i, next)));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    let mut count = movements.len();
    movements
        .iter()
        .for_each(|(cucumber, (i, j), (new_i, new_j))| {
            let old_cucumber = sea_cucumbers
                .get_mut(*i)
                .expect("Bounds Checked")
                .get_mut(*j)
                .expect("Bounds Checked");
            *old_cucumber = SeaCucumber::None;
            let new_cucumber = sea_cucumbers
                .get_mut(*new_i)
                .expect("Bounds Checked")
                .get_mut(*new_j)
                .expect("Bounds Checked");
            *new_cucumber = *cucumber;
        });

    movements = Vec::new();

    for i in 0..sea_cucumbers.len() {
        let column = sea_cucumbers.get(i).expect("Bounds checked");
        for j in 0..column.len() {
            let cucumber = column.get(j).expect("Bounds checked");
            match cucumber {
                SeaCucumber::Down => {
                    let next = if (i + 1) == sea_cucumbers.len() {
                        0usize
                    } else {
                        i + 1
                    };
                    let next_space = sea_cucumbers
                        .get(next)
                        .expect("Bounds Checked")
                        .get(j)
                        .expect("Bounds Checked");

                    match next_space {
                        SeaCucumber::None => {
                            movements.push((SeaCucumber::Down, (i, j), (next, j)));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    count += movements.len();

    movements
        .iter()
        .for_each(|(cucumber, (i, j), (new_i, new_j))| {
            let old_cucumber = sea_cucumbers
                .get_mut(*i)
                .expect("Bounds Checked")
                .get_mut(*j)
                .expect("Bounds Checked");
            *old_cucumber = SeaCucumber::None;
            let new_cucumber = sea_cucumbers
                .get_mut(*new_i)
                .expect("Bounds Checked")
                .get_mut(*new_j)
                .expect("Bounds Checked");
            *new_cucumber = *cucumber;
        });

    count
}

fn parse_data(input: &String) -> IResult<&str, Vec<Vec<SeaCucumber>>> {
    separated_list0(newline, parse_cucumber_row)(input)
}

fn parse_cucumber_row(input: &str) -> IResult<&str, Vec<SeaCucumber>> {
    many1(parse_cucumber)(input)
}

fn parse_cucumber(input: &str) -> IResult<&str, SeaCucumber> {
    alt((
        value(SeaCucumber::Right, tag(">")),
        value(SeaCucumber::Down, tag("v")),
        value(SeaCucumber::None, tag(".")),
    ))(input)
}
