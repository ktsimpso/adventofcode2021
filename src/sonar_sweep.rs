use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command, CommandResult};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches};

pub const SONAR_SWEEP: Command = Command::new(sub_command, "sonar-sweep", "day1_sonar_sweep", run);

#[derive(Debug)]
struct SonarSweepArgs {
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

fn run(arguments: &ArgMatches, file: &String) -> Result<CommandResult, Error> {
    let sonar_arguments = match arguments.subcommand_name() {
        Some("part1") => SonarSweepArgs {
            sample_size: 1,
        },
        Some("part2") => SonarSweepArgs {
            sample_size: 3,
        },
        _ => SonarSweepArgs {
            sample_size: value_t_or_exit!(arguments.value_of("sample"), usize),
        },
    };

    file_to_lines(file)
        .and_then(|lines| {
            parse_lines(lines, |line| line.parse::<isize>()).map_err(|err| err.into())
        })
        .map(|lines| aggregate_samples(&lines, &sonar_arguments.sample_size))
        .map(count_increases)
        .map(CommandResult::from)
}

fn aggregate_samples(input: &Vec<isize>, sample_size: &usize) -> Vec<isize> {
    input
        .windows(*sample_size)
        .map(|window| window.into_iter().fold(0, |acc, number| acc + number))
        .collect()
}

fn count_increases(input: Vec<isize>) -> isize {
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
