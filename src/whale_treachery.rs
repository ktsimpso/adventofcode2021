use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{bytes::complete::tag, combinator::map, multi::separated_list0, IResult};
use std::{collections::HashMap, convert::identity};
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const WHALE_TREACHERY: Problem<WhaleTreacheryArgs, HashMap<usize, usize>> = Problem::new(
    sub_command,
    "whale-treachery",
    "day7_whale_treachery",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct WhaleTreacheryArgs {
    fuel_function: FuelFunction,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum FuelFunction {
    Constant,
    Linear,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &WHALE_TREACHERY,
        "Finds the minimum fuel cost to align",
        "Path to the input file. Input should be comma delimited integers.",
        "Finds the minium fuel cost with constant fuel and the default input.",
        "Finds the minium fuel cost with linear fuel and the default input.",
    ).arg(
        Arg::with_name("fuel-function")
            .short("n")
            .help("The type of fuel consumption for the crabs. The functions available are as follows:\n\n\
            constant: Each distance from the target costs 1 fuel.\n\n\
            linear: Each distance from the target costs 1 more fuel than the previous distance.\n\n")
            .takes_value(true)
            .possible_values(&FuelFunction::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> WhaleTreacheryArgs {
    match arguments.subcommand_name() {
        Some("part1") => WhaleTreacheryArgs {
            fuel_function: FuelFunction::Constant,
        },
        Some("part2") => WhaleTreacheryArgs {
            fuel_function: FuelFunction::Linear,
        },
        _ => WhaleTreacheryArgs {
            fuel_function: value_t_or_exit!(arguments.value_of("fuel-function"), FuelFunction),
        },
    }
}

fn run(arguments: WhaleTreacheryArgs, crabs: HashMap<usize, usize>) -> CommandResult {
    let fuel_function = match arguments.fuel_function {
        FuelFunction::Constant => identity,
        FuelFunction::Linear => linear,
    };

    let min = *crabs
        .keys()
        .reduce(|min, item| if item < min { item } else { min })
        .unwrap_or(&0usize);
    let max = *crabs
        .keys()
        .fold(&0usize, |max, item| if item > max { item } else { max });
    (min..max)
        .map(|position| fuel_cost_at_position(&crabs, &position, fuel_function))
        .reduce(|min, item| if item < min { item } else { min })
        .unwrap_or(0usize)
        .into()
}

fn fuel_cost_at_position(
    crabs: &HashMap<usize, usize>,
    position: &usize,
    fuel_function: impl Fn(usize) -> usize,
) -> usize {
    crabs
        .into_iter()
        .map(|(crab, count)| {
            let n = if crab >= position {
                crab - position
            } else {
                position - crab
            };

            fuel_function(n) * count
        })
        .fold(0usize, |sum, fuel_cost| sum + fuel_cost)
}

fn linear(fuel: usize) -> usize {
    (fuel * (fuel + 1)) / 2
}

fn parse_data(input: &String) -> IResult<&str, HashMap<usize, usize>> {
    map(separated_list0(tag(","), parse_usize), |crabs| {
        crabs.into_iter().fold(HashMap::new(), |mut crabs, crab| {
            *crabs.entry(crab).or_insert(0usize) += 1;
            crabs
        })
    })(input)
}
