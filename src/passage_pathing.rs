use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, one_of},
    combinator::{map, value},
    multi::{many1, separated_list0},
    sequence::separated_pair,
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

pub const PASSAGE_PATHING: Problem<PassagePathingArgs, Vec<(Cave<'static>, Cave<'static>)>> =
    Problem::new(
        sub_command,
        "passage-pathing",
        "day12_passage_pathing",
        parse_arguments,
        parse_data,
        run,
    );

#[derive(Debug)]
pub struct PassagePathingArgs {
    reuse_small_cave: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cave<'a> {
    Start,
    End,
    Big { name: &'a str },
    Small { name: &'a str },
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Journey<'a> {
    visited_caves: HashSet<Cave<'a>>,
    caves: Vec<Cave<'a>>,
    small_cave: Option<Cave<'a>>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &PASSAGE_PATHING,
        "Goes through all possible paths and counts them",
        "Path to the input file. Input should valid transitions from cave to cave.",
        "Searches the default input for the maximum number of valid paths.",
        "Searches the default input for the maximum number but one small cave may be reused.",
    )
    .arg(
        Arg::with_name("reuse-small-cave")
            .short("r")
            .help("If passed, one small cave can be reused."),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> PassagePathingArgs {
    match arguments.subcommand_name() {
        Some("part1") => PassagePathingArgs {
            reuse_small_cave: false,
        },
        Some("part2") => PassagePathingArgs {
            reuse_small_cave: true,
        },
        _ => PassagePathingArgs {
            reuse_small_cave: arguments.is_present("reuse-small-cave"),
        },
    }
}

fn run(arguments: PassagePathingArgs, paths: Vec<(Cave<'static>, Cave<'static>)>) -> CommandResult {
    let cave_paths = paths.into_iter().fold(
        HashMap::new(),
        |mut cave_paths: HashMap<Cave<'static>, HashSet<Cave<'static>>>, (a, b)| {
            match a {
                Cave::Start => (),
                _ => match b {
                    Cave::End => (),
                    _ => {
                        cave_paths.entry(b).or_insert(HashSet::new()).insert(a);
                        ()
                    }
                },
            };
            match b {
                Cave::Start => (),
                _ => match a {
                    Cave::End => (),
                    _ => {
                        cave_paths.entry(a).or_insert(HashSet::new()).insert(b);
                        ()
                    }
                },
            };

            cave_paths
        },
    );

    let small_cave = if arguments.reuse_small_cave {
        Option::None
    } else {
        Option::Some(Cave::Start)
    };

    let mut start = Journey {
        visited_caves: HashSet::new(),
        caves: vec![Cave::Start],
        small_cave: small_cave,
    };

    start.visited_caves.insert(Cave::Start);

    find_all_journies(&cave_paths, start).len().into()
}

fn find_all_journies(
    cave_paths: &HashMap<Cave<'static>, HashSet<Cave<'static>>>,
    journey: Journey<'static>,
) -> Vec<Journey<'static>> {
    let mut journies = cave_paths
        .get(journey.caves.last().unwrap())
        .unwrap_or(&HashSet::new())
        .iter()
        .map(|cave| match cave {
            Cave::Small { name: _ } => {
                if journey.visited_caves.contains(cave) {
                    match journey.small_cave {
                        Option::Some(_) => Vec::new(),
                        Option::None => {
                            let mut new_journey = journey.clone();
                            new_journey.caves.push(*cave);
                            new_journey.small_cave = Option::Some(*cave);
                            find_all_journies(&cave_paths, new_journey)
                        }
                    }
                } else {
                    let mut new_journey = journey.clone();
                    new_journey.visited_caves.insert(*cave);
                    new_journey.caves.push(*cave);
                    find_all_journies(&cave_paths, new_journey)
                }
            }
            _ => {
                let mut new_journey = journey.clone();
                new_journey.visited_caves.insert(*cave);
                new_journey.caves.push(*cave);
                find_all_journies(&cave_paths, new_journey)
            }
        })
        .fold(Vec::new(), |mut acc, mut sub_journies| {
            acc.append(&mut sub_journies);
            acc
        });

    if journey.visited_caves.contains(&Cave::End) {
        journies.push(journey);
    }

    journies
}

fn parse_data(input: &String) -> IResult<&str, Vec<(Cave<'static>, Cave<'static>)>> {
    separated_list0(newline, parse_path)(input)
}

fn parse_path(input: &str) -> IResult<&str, (Cave<'static>, Cave<'static>)> {
    separated_pair(parse_cave, tag("-"), parse_cave)(input)
}

fn parse_cave(input: &str) -> IResult<&str, Cave<'static>> {
    alt((
        parse_start_cave,
        parse_end_cave,
        parse_big_cave,
        parse_small_cave,
    ))(input)
}

fn parse_start_cave(input: &str) -> IResult<&str, Cave<'static>> {
    value(Cave::Start, tag("start"))(input)
}

fn parse_end_cave(input: &str) -> IResult<&str, Cave<'static>> {
    value(Cave::End, tag("end"))(input)
}

fn parse_big_cave(input: &str) -> IResult<&str, Cave<'static>> {
    map(many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")), |name| {
        Cave::Big {
            name: Box::leak(String::from_iter(name.into_iter()).into_boxed_str()),
        }
    })(input)
}

fn parse_small_cave(input: &str) -> IResult<&str, Cave<'static>> {
    map(many1(one_of("abcdefghijklmnopqrstuvwxyz")), |name| {
        Cave::Small {
            name: Box::leak(String::from_iter(name.into_iter()).into_boxed_str()),
        }
    })(input)
}
