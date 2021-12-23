use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::lib::{default_sub_command, parse_isize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

pub const REACTOR_REBOOT: Problem<ReactorRebootArgs, Vec<RebootStep>> = Problem::new(
    sub_command,
    "reactor-reboot",
    "day22_reactor_reboot",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct ReactorRebootArgs {}

#[derive(Debug)]
pub struct RebootStep {
    turn_on: bool,
    cuboid: Cuboid,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cuboid {
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Range {
    low: isize,
    high: isize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &REACTOR_REBOOT,
        "Follows the reboot steps then reports the number of turned on cubes.",
        "Path to the input file. Input should be a newline delimited set of reboot instructions.",
        "Searches the default input with a sample size of 1.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> ReactorRebootArgs {
    match arguments.subcommand_name() {
        Some("part1") => ReactorRebootArgs {},
        Some("part2") => ReactorRebootArgs {},
        _ => ReactorRebootArgs {},
    }
}

fn run(arguments: ReactorRebootArgs, reboot_steps: Vec<RebootStep>) -> CommandResult {
    let filtered_steps: Vec<RebootStep> = reboot_steps
        .into_iter()
        .filter(|step| is_step_within_target(step, -50isize, 50isize))
        .collect();
    run_steps(filtered_steps).len().into()
}

fn run_steps(reboot_steps: Vec<RebootStep>) -> HashSet<Point> {
    let mut on_points = HashSet::new();

    reboot_steps.into_iter().for_each(|step| {
        for x in step.cuboid.x_range.low..=step.cuboid.x_range.high {
            for y in step.cuboid.y_range.low..=step.cuboid.y_range.high {
                for z in step.cuboid.z_range.low..=step.cuboid.z_range.high {
                    let point = Point { x: x, y: y, z: z };

                    if step.turn_on {
                        on_points.insert(point);
                    } else {
                        on_points.remove(&point);
                    };
                }
            }
        }
    });

    on_points
}

fn is_step_within_target(reboot_step: &RebootStep, low_target: isize, high_target: isize) -> bool {
    let target_cuboid = Cuboid {
        x_range: Range {
            low: low_target,
            high: high_target,
        },
        y_range: Range {
            low: low_target,
            high: high_target,
        },
        z_range: Range {
            low: low_target,
            high: high_target,
        },
    };

    match get_cuboid_intersection(&reboot_step.cuboid, &target_cuboid) {
        Some(_) => true,
        _ => false,
    }
}

fn get_cuboid_intersection(first: &Cuboid, second: &Cuboid) -> Option<Cuboid> {
    let x_intersection = get_range_intersection(&first.x_range, &second.x_range);
    let y_intersection = get_range_intersection(&first.y_range, &second.y_range);
    let z_intersection = get_range_intersection(&first.z_range, &second.z_range);

    match (x_intersection, y_intersection, z_intersection) {
        (Some(x), Some(y), Some(z)) => Option::Some(Cuboid {
            x_range: x,
            y_range: y,
            z_range: z,
        }),
        _ => Option::None,
    }
}

fn get_range_intersection(first: &Range, second: &Range) -> Option<Range> {
    let low = max(first.low, second.low);
    let high = min(first.high, second.high);
    get_range_from_low_high(low, high)
}

fn get_range_from_low_high(low: isize, high: isize) -> Option<Range> {
    if high < low {
        Option::None
    } else {
        Option::Some(Range {
            low: low,
            high: high,
        })
    }
}

fn parse_data(input: &String) -> IResult<&str, Vec<RebootStep>> {
    separated_list0(newline, parse_reboot_step)(input)
}

fn parse_reboot_step(input: &str) -> IResult<&str, RebootStep> {
    map(
        tuple((
            alt((value(true, tag("on")), value(false, tag("off")))),
            preceded(tag(" x="), parse_isize),
            preceded(tag(".."), parse_isize),
            preceded(tag(",y="), parse_isize),
            preceded(tag(".."), parse_isize),
            preceded(tag(",z="), parse_isize),
            preceded(tag(".."), parse_isize),
        )),
        |(turn_on, x_low, x_high, y_low, y_high, z_low, z_high)| RebootStep {
            turn_on: turn_on,
            cuboid: Cuboid {
                x_range: Range {
                    low: x_low,
                    high: x_high,
                },
                y_range: Range {
                    low: y_low,
                    high: y_high,
                },
                z_range: Range {
                    low: z_low,
                    high: z_high,
                },
            },
        },
    )(input)
}
