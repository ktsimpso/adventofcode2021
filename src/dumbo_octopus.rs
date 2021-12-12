use std::collections::HashSet;

use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::map_parser,
    multi::{many0, separated_list0},
    IResult,
};

pub const DUMBO_OCTOPUS: Problem<DumboOctopusArgs, Vec<Vec<usize>>> = Problem::new(
    sub_command,
    "dumbo-octopus",
    "day11_dumbo_octopus",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct DumboOctopusArgs {}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &DUMBO_OCTOPUS,
        "Simulates dumbo octopi flashing behavoir.",
        "Path to the input file. Input should be newline delimited sets of 10 integers from 0-9.",
        "Counts the number of flashes after 100 steps.",
        "I Will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> DumboOctopusArgs {
    match arguments.subcommand_name() {
        Some("part1") => DumboOctopusArgs {},
        Some("part2") => DumboOctopusArgs {},
        _ => DumboOctopusArgs {},
    }
}

fn run(arguments: DumboOctopusArgs, octopi: Vec<Vec<usize>>) -> CommandResult {
    count_flashes_after_100_steps(octopi).into()
}

fn count_flashes_after_100_steps(mut octopi: Vec<Vec<usize>>) -> usize {
    let mut flashes = 0usize;
    for _ in 0..100 {
        let (new_octopi, new_flashes) = run_step(&octopi);
        octopi = new_octopi;
        flashes += new_flashes;
    }
    flashes
}

fn run_step(octopi: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, usize) {
    let mut new_octopi: Vec<Vec<usize>> = octopi
        .iter()
        .map(|row| row.iter().map(|value| value + 1).collect())
        .collect();

    let mut flashed_octopi = HashSet::new();
    let mut has_flashes = true;

    while has_flashes {
        has_flashes = false;
        for i in 0..10usize {
            for j in 0..10usize {
                if flashed_octopi.contains(&(i, j)) {
                    continue;
                }
                let octopus = new_octopi.get(i).unwrap().get(j).unwrap();
                if *octopus > 9usize {
                    has_flashes = true;
                    flashed_octopi.insert((i, j));
                    get_adjacent_octopi((&i, &j)).iter().for_each(|(x, y)| {
                        *new_octopi.get_mut(*x).unwrap().get_mut(*y).unwrap() += 1
                    });
                }
            }
        }
    }

    (
        new_octopi
            .iter()
            .map(|row| {
                row.iter()
                    .map(|octopus| if *octopus > 9usize { 0usize } else { *octopus })
                    .collect()
            })
            .collect(),
        flashed_octopi.len(),
    )
}

fn get_adjacent_octopi(point: (&usize, &usize)) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let mut adjacents = Vec::new();
    if *y > 0usize {
        adjacents.push((*x, y - 1));
    }

    if *y < 9usize {
        adjacents.push((*x, y + 1));
    }

    if *x > 0usize {
        adjacents.push((x - 1, *y));
    }

    if *x < 9usize {
        adjacents.push((x + 1, *y));
    }

    if *x < 9usize && *y < 9usize {
        adjacents.push((x + 1, y + 1));
    }

    if *x < 9usize && *y > 0usize {
        adjacents.push((x + 1, y - 1));
    }

    if *x > 0usize && *y > 0usize {
        adjacents.push((x - 1, y - 1));
    }

    if *x > 0usize && *y < 9usize {
        adjacents.push((x - 1, y + 1));
    }

    adjacents
}

fn parse_data(input: &String) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list0(newline, parse_octopi)(input)
}

fn parse_octopi(input: &str) -> IResult<&str, Vec<usize>> {
    many0(map_parser(take(1usize), parse_usize))(input)
}
