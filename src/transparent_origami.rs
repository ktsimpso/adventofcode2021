use adventofcode2021::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::HashSet;

pub const TRANSPARENT_ORIGAMI: Problem<TransparentOrigamiArgs, Paper> = Problem::new(
    sub_command,
    "transparent-origami",
    "day13_transparent_origami",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct TransparentOrigamiArgs {
    limit_folds: bool,
}

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
        "Takes a list of dots and fold instructions, then folds the dots on themselves and displays the paper.",
        "Path to the input file. Input should be newline delimited points followed by fold instructions.",
        "Performs the first fold on the default input then counts the dots.",
        "Performs all folds on the default input then counts the dots.",
    ).arg(
        Arg::with_name("limit-folds")
            .short("l")
            .help("If passed, only the first fold is preformed."),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> TransparentOrigamiArgs {
    match arguments.subcommand_name() {
        Some("part1") => TransparentOrigamiArgs { limit_folds: true },
        Some("part2") => TransparentOrigamiArgs { limit_folds: false },
        _ => TransparentOrigamiArgs {
            limit_folds: arguments.is_present("limit-folds"),
        },
    }
}

fn run(arguments: TransparentOrigamiArgs, paper: Paper) -> CommandResult {
    let mut points = paper.points.iter().fold(HashSet::new(), |mut acc, point| {
        acc.insert(*point);
        acc
    });

    points = if arguments.limit_folds {
        fold_paper(&points, &paper.folds.first().unwrap())
    } else {
        paper
            .folds
            .iter()
            .fold(points, |acc, fold| fold_paper(&acc, fold))
    };

    display_points(&points);
    points.len().into()
}

fn display_points(points: &HashSet<Point>) -> () {
    let max_x = points.iter().map(|point| point.x).max().unwrap_or(0usize);
    let max_y = points.iter().map(|point| point.y).max().unwrap_or(0usize);

    for y in 0..=max_y {
        println!(
            "{}",
            (0..=max_x)
                .map(|x| Point { x: x, y: y })
                .map(|point| if points.contains(&point) { "#" } else { "." })
                .map(|point| point.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
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
