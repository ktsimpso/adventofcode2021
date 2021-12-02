use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};

pub const SONAR_SWEEP: Command = Command::new(sub_command, "sonar-sweep", run);

struct SonarSweepArgs {
    file: String,
    sample_size: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SONAR_SWEEP,
        "Counts every time the number in the input increases between each sample",
        "Path to the input file. Input should be newline delimited integers.",
    )
    .arg(
        Arg::with_name("sample")
            .short("s")
            .help("Number of consecttive items that must be sampled")
            .takes_value(true)
            .required(true),
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about("Searches the default input with a sample size of 1.")
            .version("1.0.0"),
    )
    .subcommand(
        SubCommand::with_name("part2")
            .about("Searches the default input with a sample size of 3.")
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let sonar_arguments = match arguments.subcommand_name() {
        Some("part1") => SonarSweepArgs {
            file: "day1_sonar_sweep/input.txt".to_string(),
            sample_size: 1,
        },
        Some("part2") => SonarSweepArgs {
            file: "day1_sonar_sweep/input.txt".to_string(),
            sample_size: 3,
        },
        _ => SonarSweepArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            sample_size: value_t_or_exit!(arguments.value_of("sample"), usize),
        },
    };

    file_to_lines(&sonar_arguments.file)
        .and_then(|lines| {
            parse_lines(lines, |line| line.parse::<isize>()).map_err(|err| err.into())
        })
        .and_then(|lines| aggregate_samples(&lines, &sonar_arguments.sample_size))
        .and_then(|lines| count_increases(&lines))
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn aggregate_samples(input: &Vec<isize>, sample_size: &usize) -> Result<Vec<isize>, Error> {
    Ok(input
        .windows(*sample_size)
        .map(|window| window.into_iter().fold(0, |acc, number| acc + number))
        .collect())
}

fn count_increases(input: &Vec<isize>) -> Result<isize, Error> {
    Ok(input.windows(2).fold(
        0,
        |sum, window| {
            if window[1] > window[0] {
                sum + 1
            } else {
                sum
            }
        },
    ))
}
