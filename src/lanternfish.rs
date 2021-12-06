use std::collections::HashMap;

use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{bytes::complete::tag, combinator::map, multi::separated_list0, IResult};

pub const LANTERNFISH: Problem<LanternfishArgs, HashMap<usize, usize>> = Problem::new(
    sub_command,
    "lanternfish",
    "day6_lanternfish",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct LanternfishArgs {
    days: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &LANTERNFISH,
        "Finds how many lantern fish there will be after a certain number of days",
        "Path to the input file. Input should be comma delimited integers",
        "Simulates the default input for 80 days",
        "Simulates the default input for 256 days",
    )
    .arg(
        Arg::with_name("days")
            .short("d")
            .help("Number of days to simulate")
            .takes_value(true)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> LanternfishArgs {
    match arguments.subcommand_name() {
        Some("part1") => LanternfishArgs { days: 80 },
        Some("part2") => LanternfishArgs { days: 256 },
        _ => LanternfishArgs {
            days: value_t_or_exit!(arguments.value_of("days"), usize),
        },
    }
}

fn run(arguments: LanternfishArgs, starting_fishes: HashMap<usize, usize>) -> CommandResult {
    let mut fishes = starting_fishes;

    for _ in 0..arguments.days {
        fishes = process_fish_day(fishes);
    }

    fishes
        .into_iter()
        .fold(0usize, |sum, (_, count)| sum + count)
        .into()
}

fn process_fish_day(fish: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let new_fish_to_add = *fish.get(&0usize).unwrap_or(&0usize);

    let mut final_fishes = fish
        .into_iter()
        .map(|(days, count)| {
            if days == 0usize {
                (6usize, count)
            } else {
                (days - 1, count)
            }
        })
        .fold(HashMap::new(), |mut fishes, (days, count)| {
            *fishes.entry(days).or_insert(0usize) += count;
            fishes
        });

    *final_fishes.entry(8).or_insert(0usize) += new_fish_to_add;
    final_fishes
}

fn parse_data(input: &String) -> IResult<&str, HashMap<usize, usize>> {
    map(separated_list0(tag(","), parse_usize), |fishes| {
        fishes.into_iter().fold(HashMap::new(), |mut fishes, fish| {
            *fishes.entry(fish).or_insert(0usize) += 1;
            fishes
        })
    })(input)
}
