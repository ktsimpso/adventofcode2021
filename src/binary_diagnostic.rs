use std::{ops::BitOr};

use crate::lib::{
    default_sub_command, file_to_lines, parse_lines, Command,
};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};

pub const BINARY_DIAGNOSTIC: Command = Command::new(sub_command, "binary-diagnostic", run);

#[derive(Debug)]
struct BinaryDiagnosticArgs {
    file: String,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &BINARY_DIAGNOSTIC,
        "Parses the binary to find diagnotics",
        "Path to the input file. Each line should contain a binary number.",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about("Finds gamma and the epsilon and multiplys them together")
            .version("1.0.0"),
    )
    /*.subcommand(
        SubCommand::with_name("part2")
            .about("I will find out")
            .version("1.0.0"),
    )*/
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let d3_arguments = match arguments.subcommand_name() {
        Some("part1") => BinaryDiagnosticArgs {
            file: "day3_binary_diagnostic/input.txt".to_string(),
        },
        Some("part2") => BinaryDiagnosticArgs {
            file: "day3_binary_diagnostic/input.txt".to_string(),
        },
        _ => BinaryDiagnosticArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    file_to_lines(&d3_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_binary))
        .map(|binary| (find_gamma(&binary), find_epsilon(&binary)))
        .map(|(gamma, epsilon)| gamma * epsilon)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn parse_binary(line: &String) -> Result<Vec<isize>, Error> {
    Ok(line
        .as_bytes()
        .into_iter()
        .map(|character| match character {
            49u8 => 1, // '1' in u8
            _ => -1,
        })
        .collect())
}

fn find_gamma(binary: &Vec<Vec<isize>>) -> usize {
    let length = binary.first().unwrap_or(&Vec::new()).len();
    binary
        .into_iter()
        .fold(vec![0; length], |accumulator, line| {
            accumulator
                .into_iter()
                .zip(line.into_iter())
                .map(|(acc, bit)| acc + bit)
                .collect()
        })
        .into_iter()
        .map(|result| match result {
            r if r > 0 => 1usize,
            _ => 0,
        })
        .fold(0usize, |acc, bit| acc.rotate_left(1).bitor(bit))
}

fn find_epsilon(binary: &Vec<Vec<isize>>) -> usize {
    let length = binary.first().unwrap_or(&Vec::new()).len();
    binary
        .into_iter()
        .fold(vec![0; length], |accumulator, line| {
            accumulator
                .into_iter()
                .zip(line.into_iter())
                .map(|(acc, bit)| acc + bit)
                .collect()
        })
        .into_iter()
        .map(|result| match result {
            r if r > 0 => 0usize,
            _ => 1usize,
        })
        .fold(0usize, |acc, bit| acc.rotate_left(1).bitor(bit))
}
