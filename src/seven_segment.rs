use adventofcode2021::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::{map, map_res},
    multi::{many0, separated_list0},
    sequence::separated_pair,
    IResult,
};
use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
};
use strum::VariantNames;
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
pub struct SevenSegmentArgs {
    decode_function: DecodeFunction,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum DecodeFunction {
    CountUniques,
    FullDecode,
}

#[derive(
    Debug, EnumString, EnumVariantNames, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord,
)]
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
    input: Vec<BTreeSet<SignalWire>>,
    output: Vec<BTreeSet<SignalWire>>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SEVEN_SEGMENT,
        "Decodes segment lines based on the signal input.",
        "Path to the input file. Input should 10 signals, followed by signal output",
        "Finds the total number of output signals that are 1, 4, 7, or 8.",
        "Parse the full signal, then sum all the outputs.",
    )
    .arg(
        Arg::with_name("decode")
            .short("d")
            .help(
                "The type of decoding requests. The functions available are as follows:\n\n\
            count-unique: Counts the total number of 1, 4, 7, and 8 signals.\n\n\
            full-decode: Fully decodes the signal then sums all the signals.\n\n",
            )
            .takes_value(true)
            .possible_values(&DecodeFunction::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SevenSegmentArgs {
    match arguments.subcommand_name() {
        Some("part1") => SevenSegmentArgs {
            decode_function: DecodeFunction::CountUniques,
        },
        Some("part2") => SevenSegmentArgs {
            decode_function: DecodeFunction::FullDecode,
        },
        _ => SevenSegmentArgs {
            decode_function: value_t_or_exit!(arguments.value_of("decode"), DecodeFunction),
        },
    }
}

fn run(arguments: SevenSegmentArgs, signal_lines: Vec<SignalLine>) -> CommandResult {
    let decode_function = match arguments.decode_function {
        DecodeFunction::CountUniques => count_1_4_7_8,
        DecodeFunction::FullDecode => concat_signal,
    };

    signal_lines
        .into_iter()
        .map(|signal_line| (decode_signals(&signal_line.input), signal_line.output))
        .map(|(decoder, output)| {
            output
                .iter()
                .map(|signal| decoder.get(signal).unwrap().to_owned())
                .collect::<Vec<usize>>()
        })
        .map(decode_function)
        .fold(0usize, |sum, line| sum + line)
        .into()
}

fn count_1_4_7_8(signal: Vec<usize>) -> usize {
    signal
        .into_iter()
        .filter(|item| *item == 1usize || *item == 4usize || *item == 7usize || *item == 8usize)
        .count()
}

fn concat_signal(signal: Vec<usize>) -> usize {
    let (_, result) = parse_usize(
        &signal
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(""),
    )
    .unwrap();

    result
}

fn decode_signals(signals: &Vec<BTreeSet<SignalWire>>) -> HashMap<BTreeSet<SignalWire>, usize> {
    let (one, four, seven, eight, rest) = find_1_4_7_8(signals);
    let (six, three, rest) = find_6_3(&one, rest);
    let (nine, rest) = find_9(&three, rest);
    let (zero, rest) = find_0(rest);
    let (five, two) = find_5_2(&six, &nine, rest);

    vec![
        (zero, 0usize),
        (one, 1usize),
        (two, 2usize),
        (three, 3usize),
        (four, 4usize),
        (five, 5usize),
        (six, 6usize),
        (seven, 7usize),
        (eight, 8usize),
        (nine, 9usize),
    ]
    .into_iter()
    .collect()
}

fn find_1_4_7_8(
    signals: &Vec<BTreeSet<SignalWire>>,
) -> (
    BTreeSet<SignalWire>,
    BTreeSet<SignalWire>,
    BTreeSet<SignalWire>,
    BTreeSet<SignalWire>,
    Vec<BTreeSet<SignalWire>>,
) {
    let one = signals
        .iter()
        .find(|segment| segment.len() == 2)
        .unwrap()
        .to_owned();
    let four = signals
        .iter()
        .find(|segment| segment.len() == 4)
        .unwrap()
        .to_owned();
    let seven = signals
        .iter()
        .find(|segment| segment.len() == 3)
        .unwrap()
        .to_owned();
    let eight = signals
        .iter()
        .find(|segment| segment.len() == 7)
        .unwrap()
        .to_owned();

    let rest = signals
        .into_iter()
        .filter(|signal| {
            **signal != one && **signal != four && **signal != seven && **signal != eight
        })
        .map(|signal| signal.to_owned())
        .collect();

    (one, four, seven, eight, rest)
}

fn find_6_3(
    one: &BTreeSet<SignalWire>,
    signals: Vec<BTreeSet<SignalWire>>,
) -> (
    BTreeSet<SignalWire>,
    BTreeSet<SignalWire>,
    Vec<BTreeSet<SignalWire>>,
) {
    let six = signals
        .iter()
        .filter(|signal| signal.len() == 6)
        .find(|signal| signal.intersection(one).count() == 1)
        .unwrap()
        .to_owned();

    let three = signals
        .iter()
        .filter(|signal| signal.len() == 5)
        .find(|signal| signal.intersection(one).count() == 2)
        .unwrap()
        .to_owned();

    let rest = signals
        .into_iter()
        .filter(|signal| *signal != six && *signal != three)
        .map(|signal| signal.to_owned())
        .collect();

    (six, three, rest)
}

fn find_9(
    three: &BTreeSet<SignalWire>,
    signals: Vec<BTreeSet<SignalWire>>,
) -> (BTreeSet<SignalWire>, Vec<BTreeSet<SignalWire>>) {
    let nine = signals
        .iter()
        .filter(|signal| signal.len() == 6)
        .find(|signal| signal.intersection(three).count() == 5)
        .unwrap()
        .to_owned();

    let rest = signals
        .into_iter()
        .filter(|signal| *signal != nine)
        .map(|signal| signal.to_owned())
        .collect();

    (nine, rest)
}

fn find_0(signals: Vec<BTreeSet<SignalWire>>) -> (BTreeSet<SignalWire>, Vec<BTreeSet<SignalWire>>) {
    let zero = signals
        .iter()
        .find(|signal| signal.len() == 6)
        .unwrap()
        .to_owned();

    let rest = signals
        .into_iter()
        .filter(|signal| *signal != zero)
        .map(|signal| signal.to_owned())
        .collect();
    (zero, rest)
}

fn find_5_2(
    six: &BTreeSet<SignalWire>,
    nine: &BTreeSet<SignalWire>,
    signals: Vec<BTreeSet<SignalWire>>,
) -> (BTreeSet<SignalWire>, BTreeSet<SignalWire>) {
    let five = six
        .intersection(nine)
        .map(|signal| signal.to_owned())
        .collect();

    let two = signals
        .into_iter()
        .find(|signal| *signal != five)
        .unwrap()
        .to_owned();

    (five, two)
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

fn parse_singals(input: &str) -> IResult<&str, Vec<BTreeSet<SignalWire>>> {
    separated_list0(tag(" "), parse_segment)(input)
}

fn parse_segment(input: &str) -> IResult<&str, BTreeSet<SignalWire>> {
    map(many0(parse_signal_wire), |signals| {
        signals.into_iter().collect()
    })(input)
}

fn parse_signal_wire(input: &str) -> IResult<&str, SignalWire> {
    map_res(take(1usize), SignalWire::from_str)(input)
}
