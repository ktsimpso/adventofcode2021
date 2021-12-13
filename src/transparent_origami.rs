use std::collections::HashSet;

use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub const TRANSPARENT_ORIGAMI: Problem<TransparentOrigamiArgs, Paper> = Problem::new(
    sub_command,
    "transparent-origami",
    "day13_transparent_origami",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct TransparentOrigamiArgs {}

#[derive(Debug)]
pub struct Paper {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Fold {
    Veritical { y: usize },
    Horizontal { x: usize },
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &TRANSPARENT_ORIGAMI,
        "Takes a list of dots and fold instructions, then folds the dots on themselves.",
        "Path to the input file. Input should be newline delimited points followed by fold instructions.",
        "Performs the first fold on the default input then counts the dots.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> TransparentOrigamiArgs {
    match arguments.subcommand_name() {
        Some("part1") => TransparentOrigamiArgs {},
        Some("part2") => TransparentOrigamiArgs {},
        _ => TransparentOrigamiArgs {},
    }
}

fn run(arguments: TransparentOrigamiArgs, paper: Paper) -> CommandResult {
    let mut points = paper.points.iter().fold(HashSet::new(), |mut acc, point| {
        acc.insert(*point);
        acc
    });

    points = fold_paper(&points, &paper.folds.first().unwrap());
    points.len().into()
}

fn fold_paper(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    match fold {
        Fold::Veritical { y } => points
            .iter()
            .map(|point| {
                if point.y > *y {
                    Point {
                        x: point.x,
                        y: y - (point.y - y),
                    }
                } else {
                    *point
                }
            })
            .collect(),
        Fold::Horizontal { x } => points
            .iter()
            .map(|point| {
                if point.x > *x {
                    Point {
                        x: x - (point.x - x),
                        y: point.y,
                    }
                } else {
                    *point
                }
            })
            .collect(),
    }
}

fn parse_data(input: &String) -> IResult<&str, Paper> {
    map(
        separated_pair(
            separated_list1(newline, parse_point),
            tag("\n\n"),
            separated_list1(newline, parse_fold),
        ),
        |(points, folds)| Paper {
            points: points,
            folds: folds,
        },
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(parse_usize, tag(","), parse_usize),
        |(x, y)| Point { x: x, y: y },
    )(input)
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    preceded(
        tag("fold along "),
        alt((
            map(
                separated_pair(tag("y"), tag("="), parse_usize),
                |(_, value)| Fold::Veritical { y: value },
            ),
            map(
                separated_pair(tag("x"), tag("="), parse_usize),
                |(_, value)| Fold::Horizontal { x: value },
            ),
        )),
    )(input)
}
