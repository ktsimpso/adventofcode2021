use std::collections::{HashMap, HashSet};

use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::map_parser,
    multi::{many1, separated_list0},
    IResult,
};

pub const CHITON: Problem<ChitonArgs, Vec<Vec<usize>>> = Problem::new(
    sub_command,
    "chiton",
    "day15_chiton",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]

struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct ChitonArgs {}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &CHITON,
        "Finds the least risky path from top left to bottom right then sums the risk.",
        "Path to the input file. Input should be newline delimited ranges of integers.",
        "Searches the default input for the path with the lowest risk level.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> ChitonArgs {
    match arguments.subcommand_name() {
        Some("part1") => ChitonArgs {},
        Some("part2") => ChitonArgs {},
        _ => ChitonArgs {},
    }
}

fn run(arguments: ChitonArgs, cavern: Vec<Vec<usize>>) -> CommandResult {
    let row_max = cavern.len();
    let column_max = cavern.first().unwrap().len();

    let mut unvisted_points = (0..row_max)
        .map(|y| (0..column_max).map(|x| Point { x: x, y: y }).collect())
        .fold(HashSet::new(), |mut acc, row: Vec<Point>| {
            acc.extend(row.into_iter());
            acc
        });

    let mut current = Point {
        x: 0usize,
        y: 0usize,
    };
    let mut costs = HashMap::new();
    costs.insert(current, 0usize);

    loop {
        get_adjacent_points(&row_max, &column_max, &current)
            .iter()
            .filter(|point| unvisted_points.contains(point))
            .map(|point| (point, cavern.get(point.y).unwrap().get(point.x).unwrap()))
            .for_each(|(point, cost)| {
                let potential_new_cost = costs.get(&current).unwrap() + *cost;
                let new_cost = match costs.get(point) {
                    Some(old_cost) => {
                        if *old_cost < potential_new_cost {
                            *old_cost
                        } else {
                            potential_new_cost
                        }
                    }
                    None => potential_new_cost,
                };
                costs.insert(*point, new_cost);
            });

        unvisted_points.remove(&current);

        let result = unvisted_points
            .iter()
            .filter(|point| costs.contains_key(point))
            .map(|point| (*costs.get(point).unwrap(), *point))
            .min();

        if let Some((_, next_point)) = result {
            current = next_point;
        } else {
            break;
        }
    }

    (*costs
        .get(&Point {
            x: column_max - 1,
            y: row_max - 1,
        })
        .unwrap_or(&0usize))
    .into()
}

fn get_adjacent_points(max_row_size: &usize, max_column_size: &usize, start: &Point) -> Vec<Point> {
    let x = start.x;
    let y = start.y;
    let mut adjacents = Vec::new();
    if y > 0usize {
        adjacents.push(Point { x: x, y: y - 1 });
    }

    if y < (*max_row_size - 1usize) {
        adjacents.push(Point { x: x, y: y + 1 });
    }

    if x > 0usize {
        adjacents.push(Point { x: x - 1, y: y });
    }

    if x < (*max_column_size - 1usize) {
        adjacents.push(Point { x: x + 1, y: y });
    }

    adjacents
}

fn parse_data(input: &String) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    many1(map_parser(take(1usize), parse_usize))(input)
}
