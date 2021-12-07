use std::collections::HashMap;

use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{bytes::complete::tag, combinator::map, multi::separated_list0, IResult};

pub const WHALE_TREACHERY: Problem<WhaleTreacheryArgs, HashMap<usize, usize>> = Problem::new(
    sub_command,
    "whale-treachery",
    "day7_whale_treachery",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct WhaleTreacheryArgs {}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &WHALE_TREACHERY,
        "Finds the minimum fuel cost to align",
        "Path to the input file. Input should be comma delimited integers.",
        "Finds the minium fuel cost with the default input.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> WhaleTreacheryArgs {
    match arguments.subcommand_name() {
        Some("part1") => WhaleTreacheryArgs {},
        Some("part2") => WhaleTreacheryArgs {},
        _ => WhaleTreacheryArgs {},
    }
}

fn run(arguments: WhaleTreacheryArgs, crabs: HashMap<usize, usize>) -> CommandResult {
    let min = crabs
        .clone()
        .into_keys()
        .reduce(|min, item| if item < min { item } else { min })
        .unwrap_or(0usize);
    let max = crabs
        .clone()
        .into_keys()
        .fold(0usize, |max, item| if item > max { item } else { max });
    (min..max)
        .map(|position| fuel_cost_at_position(&crabs, &position))
        .reduce(|min, item| if item < min { item } else { min })
        .unwrap_or(0usize)
        .into()
}

fn fuel_cost_at_position(crabs: &HashMap<usize, usize>, position: &usize) -> usize {
    crabs
        .into_iter()
        .map(|(crab, count)| {
            let n = if crab >= position {
                crab - position
            } else {
                position - crab
            };

            n * count
        })
        .fold(0usize, |sum, fuel_cost| sum + fuel_cost)
}

fn parse_data(input: &String) -> IResult<&str, HashMap<usize, usize>> {
    map(separated_list0(tag(","), parse_usize), |crabs| {
        crabs.into_iter().fold(HashMap::new(), |mut crabs, crab| {
            *crabs.entry(crab).or_insert(0usize) += 1;
            crabs
        })
    })(input)
}
