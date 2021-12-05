use crate::lib::{complete_parsing, default_sub_command, file_to_string, CommandResult, Problem};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::IResult;
use std::convert::identity;
use std::ops::{BitAnd, BitOr};
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const BINARY_DIAGNOSTIC: Problem<BinaryDiagnosticArgs> = Problem::new(
    sub_command,
    "binary-diagnostic",
    "day3_binary_diagnostic",
    parse_arguments,
    run,
);

#[derive(Debug)]
pub struct BinaryDiagnosticArgs {
    diagnostic: Diagnostic,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Diagnostic {
    PowerConsumption,
    LifeSupport,
}

#[derive(Debug, Clone, Copy)]
struct Binary {
    bits: usize,
    significant_bits: u32,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &BINARY_DIAGNOSTIC,
        "Parses the binary to find diagnotics",
        "Path to the input file. Each line should contain a binary number.",
        "Finds power consumption.",
        "Finds the life support rating",
    )
    .arg(
        Arg::with_name("diagnostic")
            .short("d")
            .help("The diagnostic requested. The diagnostics available are as follows:\n\n\
            power-consumption: Finds the gamma rate and the epsilon rate and multiplies them.\n\n\
            life-support: Finds the oxygen rating and the CO2 scrubber rating and multiplies them.\n\n")
            .takes_value(true)
            .possible_values(&Diagnostic::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> BinaryDiagnosticArgs {
    match arguments.subcommand_name() {
        Some("part1") => BinaryDiagnosticArgs {
            diagnostic: Diagnostic::PowerConsumption,
        },
        Some("part2") => BinaryDiagnosticArgs {
            diagnostic: Diagnostic::LifeSupport,
        },
        _ => BinaryDiagnosticArgs {
            diagnostic: value_t_or_exit!(arguments.value_of("diagnostic"), Diagnostic),
        },
    }
}

fn run(arguments: &BinaryDiagnosticArgs, file: &String) -> Result<CommandResult, Error> {
    file_to_string(&file)
        .and_then(|lines| complete_parsing(parse_binary)(&lines))
        .map(|binary| match arguments.diagnostic {
            Diagnostic::PowerConsumption => (find_gamma(&binary), find_epsilon(&binary)),
            Diagnostic::LifeSupport => (find_oxygen(&binary), find_c02(&binary)),
        })
        .map(|(metric1, metric2)| metric1 * metric2)
        .map(CommandResult::from)
}

fn parse_binary(file: &String) -> IResult<&str, Vec<Binary>> {
    separated_list0(
        newline,
        map_res(
            map_res(take_until("\n"), |line| {
                usize::from_str_radix(line, 2).map(|bits| (bits, line))
            }),
            |(bits, line)| {
                line.len().try_into().map(|length| Binary {
                    bits: bits,
                    significant_bits: length,
                })
            },
        ),
    )(file)
}

fn most_common_bit_at_position(numbers: &Vec<Binary>, position: u32) -> usize {
    let mask = 1usize.rotate_left(position);
    let bits: Vec<usize> = numbers
        .into_iter()
        .map(|bin| bin.bits)
        .map(|number| number.bitand(mask))
        .map(|number| number.rotate_right(position))
        .collect();
    let ones = bits.into_iter().filter(|bit| bit == &1usize).count();
    let zeros = numbers.len() - ones;
    if ones >= zeros {
        1
    } else {
        0
    }
}

fn most_to_least(bit: usize) -> usize {
    if bit == 1usize {
        0
    } else {
        1
    }
}

fn find_gamma(binary: &Vec<Binary>) -> usize {
    combine_common_bits(binary, identity)
}

fn find_epsilon(binary: &Vec<Binary>) -> usize {
    combine_common_bits(binary, most_to_least)
}

fn combine_common_bits(binary: &Vec<Binary>, convert_function: impl Fn(usize) -> usize) -> usize {
    let most_significant = binary.first().map(|bin| bin.significant_bits).unwrap_or(0);
    (0..most_significant)
        .map(|position| {
            let common = convert_function(most_common_bit_at_position(binary, position));
            common.rotate_left(position)
        })
        .fold(0usize, |acc, bit| acc.bitor(bit))
}

fn find_oxygen(binary: &Vec<Binary>) -> usize {
    filter_by_significant_bits(binary, identity)
}

fn find_c02(binary: &Vec<Binary>) -> usize {
    filter_by_significant_bits(binary, most_to_least)
}

fn filter_by_significant_bits(
    binary: &Vec<Binary>,
    convert_function: impl Fn(usize) -> usize,
) -> usize {
    let most_significant = binary.first().map(|bin| bin.significant_bits).unwrap_or(0);
    let mut position = most_significant;
    let mut filtered_binary = binary.clone();

    while filtered_binary.len() > 1 {
        position -= 1;
        let common = convert_function(most_common_bit_at_position(&filtered_binary, position));
        let mask = 1usize.rotate_left(position);
        filtered_binary = filtered_binary
            .into_iter()
            .filter(|bits| bits.bits.bitand(mask).rotate_right(position) == common)
            .collect();
    }

    filtered_binary
        .first()
        .map(|bin| bin.bits)
        .unwrap_or(0usize)
}
