use crate::lib::{default_sub_command, parse_isize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
use num_integer::Roots;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const TRICK_SHOT: Problem<TrickShotArgs, Target> = Problem::new(
    sub_command,
    "trick-shot",
    "day17_trick_shot",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct TrickShotArgs {
    metric: Metric,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Metric {
    MaxHeight,
    TrajectoryCount,
}

#[derive(Debug)]
pub struct Target {
    lower_x: isize,
    upper_x: isize,
    lower_y: isize,
    upper_y: isize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &TRICK_SHOT,
        "Calculates valid trajectories for a target.",
        "Path to the input file. Input should be the target area.",
        "Finds the maximum height that can be acheived while still hitting the target for the default input.",
        "Finds the total number of valid trajectories with the default input.",
    ).arg(
        Arg::with_name("metric")
            .short("m")
            .help(
                "The type of metric to calculate. The functions available are as follows:\n\n\
            max-height: Counts height for any trajectory to hit a target.\n\n\
            trajectory-count: Counts the total number of valid trajectories for the target.\n\n",
            )
            .takes_value(true)
            .possible_values(&Metric::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> TrickShotArgs {
    match arguments.subcommand_name() {
        Some("part1") => TrickShotArgs {
            metric: Metric::MaxHeight,
        },
        Some("part2") => TrickShotArgs {
            metric: Metric::TrajectoryCount,
        },
        _ => TrickShotArgs {
            metric: value_t_or_exit!(arguments.value_of("metric"), Metric),
        },
    }
}

fn run(arguments: TrickShotArgs, target: Target) -> CommandResult {
    match arguments.metric {
        Metric::MaxHeight => find_max_possible_height(&target).into(),
        Metric::TrajectoryCount => find_all_valid_trajectories(&target).len().into(),
    }
}

fn find_all_valid_trajectories(target: &Target) -> Vec<(isize, isize)> {
    let mut valid_trajectories = Vec::new();

    for x in find_min_possible_x(&target)..=find_max_possible_x(&target) {
        for y in find_min_possible_y(&target)..=find_max_possible_y(&target) {
            if is_valid_trajectory(&x, &y, &target) {
                valid_trajectories.push((x, y));
            }
        }
    }

    valid_trajectories
}

fn find_max_possible_height(target: &Target) -> isize {
    max_y(&find_max_possible_y(&target))
}

fn is_valid_trajectory(x: &isize, y: &isize, target: &Target) -> bool {
    let mut n = 0isize;
    let mut valid = false;

    loop {
        let x_n = x_at_n(x, &n);
        let y_n = y_at_n(y, &n);

        if x_n > target.upper_x || y_n < target.lower_y {
            break;
        } else if x_n >= target.lower_x && y_n <= target.upper_y {
            valid = true;
            break;
        }

        n += 1
    }

    valid
}

fn find_max_possible_y(target: &Target) -> isize {
    // technically if target.lower_y > 0 it would just be target.lower_y
    target.lower_y.abs() - 1
}

fn find_min_possible_y(target: &Target) -> isize {
    target.lower_y
}

fn find_max_possible_x(target: &Target) -> isize {
    target.upper_x
}

fn find_min_possible_x(target: &Target) -> isize {
    // should be ((target.lower_x * 8 + 1).sqrt()) - 1 / 2 (hard coding to the positive root)
    // but since we are getting the integer sqrt we can omit it
    ((target.lower_x * 8 + 1).sqrt()) / 2
}

fn x_at_n(x: &isize, n: &isize) -> isize {
    if n >= x {
        max_y(x)
    } else {
        y_at_n(x, n)
    }
}

fn max_y(y: &isize) -> isize {
    y_at_n(&y, &*y)
}

fn y_at_n(y: &isize, n: &isize) -> isize {
    ((2isize * y + 1isize) * n - (n * n)) / 2
}

fn parse_data(input: &String) -> IResult<&str, Target> {
    map(
        tuple((
            preceded(tag("target area: x="), parse_isize),
            preceded(tag(".."), parse_isize),
            preceded(tag(", y="), parse_isize),
            preceded(tag(".."), parse_isize),
        )),
        |(lower_x, upper_x, lower_y, upper_y)| Target {
            lower_x: lower_x,
            upper_x: upper_x,
            lower_y: lower_y,
            upper_y: upper_y,
        },
    )(input)
}
