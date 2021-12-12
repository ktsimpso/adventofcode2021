use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{flat_map, map, opt, value},
    multi::{many0, many1, separated_list0},
    sequence::tuple,
    IResult,
};
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const SYNTAX_SCORING: Problem<SyntaxScoringArgs, Vec<Vec<Chunk>>> = Problem::new(
    sub_command,
    "syntax-scoring",
    "day10_syntax_scoring",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct SyntaxScoringArgs {
    scoring_function: ScoringFunction,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum ScoringFunction {
    Corrupted,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bracket {
    Paren,
    Square,
    Curly,
    Angle,
}

#[derive(Debug)]
pub enum Chunk {
    CorruptedChunk {
        first: Bracket,
        chunks: Vec<Chunk>,
        invalid: Bracket,
    },
    IncompleteChunk {
        first: Bracket,
        chunks: Vec<Chunk>,
    },
    CompleteChunk {
        bracket: Bracket,
        chunks: Vec<Chunk>,
    },
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SYNTAX_SCORING,
        "Parses chunks from lines of chunks then calculates stats based on the result.",
        "Path to the input file. Input should be newline delimited chunks.",
        "Searches the default and scores all the lines with corrupted chunks.",
        "Searches the default, scores the incomplete lines and returns the middle valued one.",
    )
    .arg(
        Arg::with_name("scoring-function")
            .short("s")
            .help(
                "The scoring type to use. The functions available are as follows:\n\n\
            corrupted: Scores the first Corrupted Chunk of each line then sums the scores.\n\n\
            incomplete: Scores the non Corrupted Incomplete Lines returns the middle score.\n\n",
            )
            .takes_value(true)
            .possible_values(&ScoringFunction::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SyntaxScoringArgs {
    match arguments.subcommand_name() {
        Some("part1") => SyntaxScoringArgs {
            scoring_function: ScoringFunction::Corrupted,
        },
        Some("part2") => SyntaxScoringArgs {
            scoring_function: ScoringFunction::Incomplete,
        },
        _ => SyntaxScoringArgs {
            scoring_function: value_t_or_exit!(
                arguments.value_of("scoring-function"),
                ScoringFunction
            ),
        },
    }
}

fn run(arguments: SyntaxScoringArgs, chunk_lines: Vec<Vec<Chunk>>) -> CommandResult {
    match arguments.scoring_function {
        ScoringFunction::Corrupted => sum_corrupted_chunks(chunk_lines),
        ScoringFunction::Incomplete => middle_incomplete_chunk_score(chunk_lines),
    }
    .into()
}

fn middle_incomplete_chunk_score(chunk_lines: Vec<Vec<Chunk>>) -> usize {
    let mut result: Vec<usize> = chunk_lines
        .iter()
        .filter(|chunks| {
            chunks
                .iter()
                .map(evaluate_corrupt_chunks)
                .all(|value| value == 0)
        })
        .map(|chunks| {
            chunks
                .iter()
                .map(evaluate_incomplete_chunks)
                .fold(0usize, |acc, score| acc + score)
        })
        .collect();
    result.sort();
    *result.get(result.len() / 2).unwrap()
}

fn evaluate_incomplete_chunks(chunk: &Chunk) -> usize {
    list_missing_brackets(&chunk)
        .iter()
        .fold(0usize, |mut acc, bracket| {
            acc *= 5;
            acc + match bracket {
                Bracket::Paren => 1usize,
                Bracket::Square => 2usize,
                &Bracket::Curly => 3usize,
                &Bracket::Angle => 4usize,
            }
        })
}

fn list_missing_brackets(chunk: &Chunk) -> Vec<Bracket> {
    match chunk {
        Chunk::CompleteChunk { bracket: _, chunks } => chunks
            .iter()
            .map(list_missing_brackets)
            .fold(Vec::new(), |mut acc, mut brackets| {
                acc.append(&mut brackets);
                acc
            }),
        Chunk::IncompleteChunk { first, chunks } => {
            let mut result = chunks.iter().map(list_missing_brackets).fold(
                Vec::new(),
                |mut acc, mut brackets| {
                    acc.append(&mut brackets);
                    acc
                },
            );
            result.push(*first);
            result
        }
        _ => Vec::new(),
    }
}

fn sum_corrupted_chunks(chunk_lines: Vec<Vec<Chunk>>) -> usize {
    chunk_lines
        .iter()
        .map(|chunks| {
            chunks
                .iter()
                .map(evaluate_corrupt_chunks)
                .filter(|value| value > &0usize)
                .next()
                .unwrap_or(0usize)
        })
        .fold(0usize, |sum, invalid_score| sum + invalid_score)
}

fn evaluate_corrupt_chunks(chunk: &Chunk) -> usize {
    match chunk {
        Chunk::CompleteChunk { bracket: _, chunks } => chunks
            .iter()
            .map(evaluate_corrupt_chunks)
            .filter(|value| value > &0usize)
            .next()
            .unwrap_or(0usize),
        Chunk::IncompleteChunk { first: _, chunks } => chunks
            .iter()
            .map(evaluate_corrupt_chunks)
            .filter(|value| value > &0usize)
            .next()
            .unwrap_or(0usize),
        Chunk::CorruptedChunk {
            first: _,
            chunks,
            invalid,
        } => chunks
            .iter()
            .map(evaluate_corrupt_chunks)
            .filter(|value| value > &0usize)
            .next()
            .unwrap_or(match invalid {
                Bracket::Paren => 3usize,
                Bracket::Square => 57usize,
                Bracket::Curly => 1197usize,
                Bracket::Angle => 25137usize,
            }),
    }
}

fn parse_data(input: &String) -> IResult<&str, Vec<Vec<Chunk>>> {
    separated_list0(newline, many1(parse_chunk))(input)
}

fn parse_chunk(input: &str) -> IResult<&str, Chunk> {
    flat_map(
        alt((
            value(Bracket::Paren, tag("(")),
            value(Bracket::Square, tag("[")),
            value(Bracket::Curly, tag("{")),
            value(Bracket::Angle, tag("<")),
        )),
        parse_rest_of_chunk,
    )(input)
}

fn parse_rest_of_chunk(first: Bracket) -> impl FnMut(&str) -> IResult<&str, Chunk> {
    move |input: &str| {
        map(
            tuple((
                many0(parse_chunk),
                opt(alt((
                    value(Bracket::Paren, tag(")")),
                    value(Bracket::Square, tag("]")),
                    value(Bracket::Curly, tag("}")),
                    value(Bracket::Angle, tag(">")),
                ))),
            )),
            |(chunks, bracket)| match bracket {
                Some(b) => match b {
                    _ if b == first => Chunk::CompleteChunk {
                        bracket: b,
                        chunks: chunks,
                    },
                    _ => Chunk::CorruptedChunk {
                        first: first,
                        chunks: chunks,
                        invalid: b,
                    },
                },
                None => Chunk::IncompleteChunk {
                    first: first,
                    chunks: chunks,
                },
            },
        )(input)
    }
}
