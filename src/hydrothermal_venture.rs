use std::{collections::{HashMap, HashSet}, convert::identity};

use crate::lib::{
    complete_parsing, default_sub_command, file_to_string, parse_usize, Command, CommandResult,
};
use anyhow::Error;
use clap::{App, Arg, ArgMatches};
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list0, sequence::separated_pair,
    IResult,
};

pub const HYDROTHERMAL_VENTURE: Command = Command::new(
    sub_command,
    "hydrothermal-venture",
    "day5_hydrothermal_venture",
    run,
);

#[derive(Debug)]
struct HydrothermalVentureArgs {
    ignore_diagnal_lines: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &HYDROTHERMAL_VENTURE,
        "Counts the number of overlapping lines of hydrothermal vents",
        "Path to the input file. Input should be pairs of coordinates",
        "Searches the default input ignoring diagnal lines and finds the number of overlapping vents.",
        "Searches the default input and finds the number of overlapping vents.",
    )
    .arg(
        Arg::with_name("ignore-diagnal-lines")
        .short("i")
        .help("If passed, ignore diagnal lines when mapping vents"))
}

fn run(arguments: &ArgMatches, file: &String) -> Result<CommandResult, Error> {
    let hydrothermal_arguments = match arguments.subcommand_name() {
        Some("part1") => HydrothermalVentureArgs { ignore_diagnal_lines: true },
        Some("part2") => HydrothermalVentureArgs { ignore_diagnal_lines: false },
        _ => HydrothermalVentureArgs {
            ignore_diagnal_lines: arguments.is_present("ignore-diagnal-lines"),
        },
    };

    let filter = if hydrothermal_arguments.ignore_diagnal_lines {
        filter_horizontal_and_vertical_lines
    } else {
        identity
    };

    file_to_string(file)
        .and_then(|file_content| complete_parsing(parse_all_lines)(&file_content))
        .map(filter)
        .map(|lines| find_overlapping_points(&lines))
        .map(CommandResult::from)
}

fn find_overlapping_points(lines: &Vec<Line>) -> usize {
    overlap_vents(&(lines.into_iter().map(expand_line_into_points).collect()))
        .into_iter()
        .filter(|(_, count)| count > &1)
        .count()
}

fn overlap_vents(vents: &Vec<HashSet<Point>>) -> HashMap<Point, usize> {
    let mut coordinates = HashMap::new();

    vents.into_iter().for_each(|vent| {
        vent.into_iter()
            .for_each(|point| *coordinates.entry(*point).or_insert(0usize) += 1)
    });

    coordinates
}

fn expand_line_into_points(line: &Line) -> HashSet<Point> {
    let mut points = HashSet::new();
    let mut x = line.start.x;
    let mut y = line.start.y;

    while x != line.end.x || y != line.end.y {
        points.insert(Point { x: x, y: y });

        if x > line.end.x {
            x -= 1;
        } else if x < line.end.x {
            x += 1;
        }

        if y > line.end.y {
            y -= 1;
        } else if y < line.end.y {
            y += 1;
        }
    }

    points.insert(Point {
        x: line.end.x,
        y: line.end.y,
    });

    points
}

fn filter_horizontal_and_vertical_lines(lines: Vec<Line>) -> Vec<Line> {
    lines
        .into_iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect()
}

fn parse_all_lines(input: &String) -> IResult<&str, Vec<Line>> {
    separated_list0(tag("\n"), parse_line)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(parse_usize, tag(","), parse_usize),
        |(x, y)| Point { x: x, y: y },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        separated_pair(parse_point, tag(" -> "), parse_point),
        |(start, end)| Line {
            start: start,
            end: end,
        },
    )(input)
}
