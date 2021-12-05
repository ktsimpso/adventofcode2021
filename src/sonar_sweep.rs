use crate::lib::{complete_parsing, default_sub_command, file_to_string, CommandResult, Problem};
use adventofcode2021::parse_usize;
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{character::complete::newline, multi::separated_list0, IResult};

pub const SONAR_SWEEP: Problem<SonarSweepArgs> = Problem::new(
    sub_command,
    "sonar-sweep",
    "day1_sonar_sweep",
    parse_arguments,
    run,
);

#[derive(Debug)]
pub struct SonarSweepArgs {
    sample_size: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SONAR_SWEEP,
        "Counts every time the number in the input increases between each sample",
        "Path to the input file. Input should be newline delimited integers.",
        "Searches the default input with a sample size of 1.",
        "Searches the default input with a sample size of 3.",
    )
    .arg(
        Arg::with_name("sample")
            .short("s")
            .help("Number of consecttive items that must be sampled")
            .takes_value(true)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SonarSweepArgs {
    match arguments.subcommand_name() {
        Some("part1") => SonarSweepArgs { sample_size: 1 },
        Some("part2") => SonarSweepArgs { sample_size: 3 },
        _ => SonarSweepArgs {
            sample_size: value_t_or_exit!(arguments.value_of("sample"), usize),
        },
    }
}

fn run(arguments: &SonarSweepArgs, file: &String) -> Result<CommandResult, Error> {
    file_to_string(file)
        .and_then(|lines| complete_parsing(parse_data)(&lines))
        .map(|lines| aggregate_samples(&lines, &arguments.sample_size))
        .map(count_increases)
        .map(CommandResult::from)
}

fn parse_data(input: &String) -> IResult<&str, Vec<usize>> {
    separated_list0(newline, parse_usize)(input)
}

fn aggregate_samples(input: &Vec<usize>, sample_size: &usize) -> Vec<usize> {
    input
        .windows(*sample_size)
        .map(|window| window.into_iter().fold(0, |acc, number| acc + number))
        .collect()
}

fn count_increases(input: Vec<usize>) -> usize {
    input.windows(2).fold(
        0,
        |sum, window| {
            if window[1] > window[0] {
                sum + 1
            } else {
                sum
            }
        },
    )
}
