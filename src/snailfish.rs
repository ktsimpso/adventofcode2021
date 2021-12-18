use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::cmp;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const SNAILFISH: Problem<SnailfishArgs, Vec<Pair>> = Problem::new(
    sub_command,
    "snailfish",
    "day18_snailfish",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct SnailfishArgs {
    question: Question,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Question {
    SumAll,
    MaxSum,
}

#[derive(Debug, Clone)]
enum SnailNumber {
    Literal(usize),
    Number(Box<Pair>),
}

#[derive(Debug, Clone)]
pub struct Pair {
    left: SnailNumber,
    right: SnailNumber,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SNAILFISH,
        "Takes a list of pair numbers then performs calculations on them.",
        "Path to the input file. Input should be newline delimited pairs.",
        "Sums all the pairs, then finds the magnitude for the default input.",
        "Sums each combination of pair then finds the maximum magnitude for the default input.",
    ).arg(
        Arg::with_name("question")
            .short("q")
            .help(
                "The question to answer requests. The questions available are as follows:\n\n\
            sum-all: Sums all pairs, then finds the magnitude.\n\n\
            max-sum: Checks each combination of pairs and returns the magnitude of the max sum.\n\n",
            )
            .takes_value(true)
            .possible_values(&Question::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SnailfishArgs {
    match arguments.subcommand_name() {
        Some("part1") => SnailfishArgs {
            question: Question::SumAll,
        },
        Some("part2") => SnailfishArgs {
            question: Question::MaxSum,
        },
        _ => SnailfishArgs {
            question: value_t_or_exit!(arguments.value_of("question"), Question),
        },
    }
}

fn run(arguments: SnailfishArgs, pairs: Vec<Pair>) -> CommandResult {
    match arguments.question {
        Question::SumAll => pairs
            .into_iter()
            .reduce(add)
            .map(|pair| magnitude(&pair))
            .unwrap(),
        Question::MaxSum => {
            let mut max = 0usize;
            for i in 0..(pairs.len() - 1usize) {
                for j in i..pairs.len() {
                    let first = magnitude(&add(pairs[i].clone(), pairs[j].clone()));
                    let second = magnitude(&add(pairs[j].clone(), pairs[i].clone()));
                    max = cmp::max(first, max);
                    max = cmp::max(second, max);
                }
            }

            max
        }
    }
    .into()
}

fn add(left: Pair, right: Pair) -> Pair {
    let mut pair = Pair {
        left: SnailNumber::Number(Box::new(left)),
        right: SnailNumber::Number(Box::new(right)),
    };

    loop {
        let (result, did_explode, _, _) = explode(pair, 0usize);
        pair = result;

        if did_explode {
            continue;
        }

        let (result, did_split) = split(pair);
        pair = result;

        if did_split {
            continue;
        }

        break;
    }

    pair
}

fn explode(pair: Pair, depth: usize) -> (Pair, bool, Option<usize>, Option<usize>) {
    let (mut left, did_explode, left_carry, right_carry) = explode_snail_number(pair.left, &depth);

    if did_explode {
        let right = match pair.right {
            SnailNumber::Literal(value) => match right_carry {
                Option::Some(carry) => SnailNumber::Literal(value + carry),
                Option::None => SnailNumber::Literal(value),
            },
            SnailNumber::Number(pair) => match right_carry {
                Option::Some(carry) => {
                    SnailNumber::Number(Box::new(add_to_first_available_left(*pair, carry)))
                }
                Option::None => SnailNumber::Number(pair),
            },
        };

        return (
            Pair {
                left: left,
                right: right,
            },
            true,
            left_carry,
            Option::None,
        );
    }

    let (right, did_explode, left_carry, right_carry) = explode_snail_number(pair.right, &depth);

    left = match left_carry {
        Option::Some(value) => add_to_furthest_available_right(left, value),
        Option::None => left,
    };

    (
        Pair {
            left: left,
            right: right,
        },
        did_explode,
        Option::None,
        right_carry,
    )
}

fn explode_snail_number(
    snail_number: SnailNumber,
    depth: &usize,
) -> (SnailNumber, bool, Option<usize>, Option<usize>) {
    match snail_number {
        SnailNumber::Literal(value) => (
            SnailNumber::Literal(value),
            false,
            Option::None,
            Option::None,
        ),
        SnailNumber::Number(value) => {
            if depth == &3usize {
                (
                    SnailNumber::Literal(0usize),
                    true,
                    Option::Some(match value.left {
                        SnailNumber::Literal(value) => value,
                        _ => 0usize,
                    }),
                    Option::Some(match value.right {
                        SnailNumber::Literal(value) => value,
                        _ => 0usize,
                    }),
                )
            } else {
                let (result, did_explode, left_carry, right_carry) = explode(*value, depth + 1);
                (
                    SnailNumber::Number(Box::new(result)),
                    did_explode,
                    left_carry,
                    right_carry,
                )
            }
        }
    }
}

fn add_to_first_available_left(pair: Pair, carry: usize) -> Pair {
    let left = match pair.left {
        SnailNumber::Literal(value) => SnailNumber::Literal(value + carry),
        SnailNumber::Number(value) => {
            SnailNumber::Number(Box::new(add_to_first_available_left(*value, carry)))
        }
    };

    Pair {
        left: left,
        right: pair.right,
    }
}

fn add_to_furthest_available_right(snail_number: SnailNumber, carry: usize) -> SnailNumber {
    match snail_number {
        SnailNumber::Literal(value) => SnailNumber::Literal(value + carry),
        SnailNumber::Number(value) => SnailNumber::Number(Box::new(Pair {
            left: value.left,
            right: add_to_furthest_available_right(value.right, carry),
        })),
    }
}

fn split(pair: Pair) -> (Pair, bool) {
    let (left, did_split) = split_snail_number(pair.left);

    if did_split {
        return (
            Pair {
                left: left,
                right: pair.right,
            },
            true,
        );
    };

    let (right, did_split) = split_snail_number(pair.right);

    (
        Pair {
            left: left,
            right: right,
        },
        did_split,
    )
}

fn split_snail_number(snail_number: SnailNumber) -> (SnailNumber, bool) {
    match snail_number {
        SnailNumber::Literal(value) => {
            if value > 9usize {
                let remainder = value % 2;
                (
                    SnailNumber::Number(Box::new(Pair {
                        left: SnailNumber::Literal(value / 2usize),
                        right: SnailNumber::Literal(value / 2usize + remainder),
                    })),
                    true,
                )
            } else {
                (SnailNumber::Literal(value), false)
            }
        }
        SnailNumber::Number(value) => {
            let (result, did_split) = split(*value);
            (SnailNumber::Number(Box::new(result)), did_split)
        }
    }
}

fn magnitude(pair: &Pair) -> usize {
    let left = 3 * magnitude_snail_number(&pair.left);
    let right = 2 * magnitude_snail_number(&pair.right);

    left + right
}

fn magnitude_snail_number(snail_number: &SnailNumber) -> usize {
    match &snail_number {
        SnailNumber::Literal(value) => *value,
        SnailNumber::Number(value) => magnitude(value),
    }
}

fn parse_data(input: &String) -> IResult<&str, Vec<Pair>> {
    separated_list0(newline, parse_pair)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    map(
        separated_pair(
            preceded(tag("["), parse_snail_number),
            tag(","),
            terminated(parse_snail_number, tag("]")),
        ),
        |(left, right)| Pair {
            left: left,
            right: right,
        },
    )(input)
}

fn parse_snail_number(input: &str) -> IResult<&str, SnailNumber> {
    alt((
        map(parse_usize, |value| SnailNumber::Literal(value)),
        map(parse_pair, |value| SnailNumber::Number(Box::new(value))),
    ))(input)
}
