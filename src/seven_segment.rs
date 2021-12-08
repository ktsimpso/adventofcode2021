use std::str::FromStr;

use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::{map, map_res},
    multi::{many0, separated_list0},
    sequence::separated_pair,
    IResult,
};
use strum_macros::{EnumString, EnumVariantNames};

pub const SEVEN_SEGMENT: Problem<SevenSegmentArgs, Vec<SignalLine>> = Problem::new(
    sub_command,
    "seven-segment",
    "day8_seven_segment",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct SevenSegmentArgs {}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum SignalWire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
pub struct SignalLine {
    input: Vec<Vec<SignalWire>>,
    output: Vec<Vec<SignalWire>>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SEVEN_SEGMENT,
        "Decodes segment lines based on the signal input.",
        "Path to the input file. Input should 10 signals, followed by signal output",
        "Finds the total number of output signals that are 1, 4, 7, or 8.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SevenSegmentArgs {
    match arguments.subcommand_name() {
        Some("part1") => SevenSegmentArgs {},
        Some("part2") => SevenSegmentArgs {},
        _ => SevenSegmentArgs {},
    }
}

fn run(arguments: SevenSegmentArgs, singal_lines: Vec<SignalLine>) -> CommandResult {
    singal_lines
        .into_iter()
        .map(|line| line.output)
        .map(|output| {
            let result: Vec<Vec<SignalWire>> = output
                .into_iter()
                .filter(|segment| {
                    let n = segment.len();
                    n == 2 || n == 4 || n == 3 || n == 7
                })
                .collect();
            result
        })
        .map(|output| output.len())
        .fold(0usize, |sum, item| sum + item)
        .into()
}

fn parse_data(input: &String) -> IResult<&str, Vec<SignalLine>> {
    separated_list0(newline, parse_singal_line)(input)
}

fn parse_singal_line(input: &str) -> IResult<&str, SignalLine> {
    map(
        separated_pair(parse_singals, tag("| "), parse_singals),
        |(i, output)| SignalLine {
            input: i,
            output: output,
        },
    )(input)
}

fn parse_singals(input: &str) -> IResult<&str, Vec<Vec<SignalWire>>> {
    separated_list0(tag(" "), parse_segment)(input)
}

fn parse_segment(input: &str) -> IResult<&str, Vec<SignalWire>> {
    many0(parse_signal_wire)(input)
}

fn parse_signal_wire(input: &str) -> IResult<&str, SignalWire> {
    map_res(take(1usize), SignalWire::from_str)(input)
}
