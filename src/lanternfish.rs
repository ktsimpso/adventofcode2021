use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{bytes::complete::tag, multi::separated_list0, IResult};

pub const LANTERNFISH: Problem<LanternfishArgs, Vec<usize>> = Problem::new(
    sub_command,
    "lanternfish",
    "day6_lanternfish",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct LanternfishArgs {}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &LANTERNFISH,
        "Finds how many lantern fish there will be after a certain number of days",
        "Path to the input file. Input should be comma delimited integers",
        "Searches the default input with 80 days",
        "TODO",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> LanternfishArgs {
    match arguments.subcommand_name() {
        Some("part1") => LanternfishArgs {},
        Some("part2") => LanternfishArgs {},
        _ => LanternfishArgs {},
    }
}

fn run(arguments: LanternfishArgs, starting_fishes: Vec<usize>) -> CommandResult {
    let mut fishes = starting_fishes;

    for _ in 0..80 {
        fishes = process_fish_day(fishes);
    }

    fishes.len().into()
}

fn process_fish_day(fish: Vec<usize>) -> Vec<usize> {
    let new_fish_to_add = fish
        .iter()
        .filter(|day_value| **day_value == 0usize)
        .count();

    let mut final_fishes: Vec<usize> = fish
        .into_iter()
        .map(|day_value| {
            if day_value == 0usize {
                6usize
            } else {
                day_value - 1
            }
        })
        .collect();

    final_fishes.append(&mut vec![8usize; new_fish_to_add]);
    final_fishes
}

fn parse_data(input: &String) -> IResult<&str, Vec<usize>> {
    separated_list0(tag(","), parse_usize)(input)
}
