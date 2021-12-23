use crate::lib::{default_sub_command, parse_isize, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};
use std::{
    cmp::{max, min},
    collections::HashSet,
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
pub struct ReactorRebootArgs {
    limit_cubes: bool,
}

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

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &REACTOR_REBOOT,
        "Follows the reboot steps then reports the number of turned on cubes.",
        "Path to the input file. Input should be a newline delimited set of reboot instructions.",
        "Finds all on cubes within a limited region for the default input.",
        "Finds all on cubes for the default input.",
    )
    .arg(
        Arg::with_name("limit-cubes")
            .short("l")
            .help("If passed, limits the area considered to -50, 50 for all dimensions."),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> ReactorRebootArgs {
    match arguments.subcommand_name() {
        Some("part1") => ReactorRebootArgs { limit_cubes: true },
        Some("part2") => ReactorRebootArgs { limit_cubes: false },
        _ => ReactorRebootArgs {
            limit_cubes: arguments.is_present("limit-cubes"),
        },
    }
}

fn run(arguments: ReactorRebootArgs, reboot_steps: Vec<RebootStep>) -> CommandResult {
    let filtered_steps: Vec<RebootStep> = if arguments.limit_cubes {
        reboot_steps
            .into_iter()
            .filter(|step| is_step_within_target(step, -50isize, 50isize))
            .collect()
    } else {
        reboot_steps
    };

    run_steps(filtered_steps)
        .iter()
        .map(get_cuboid_size)
        .fold(0isize, |acc, value| acc + value)
        .into()
}

fn run_steps(reboot_steps: Vec<RebootStep>) -> HashSet<Cuboid> {
    let mut on_cubes = HashSet::new();

    reboot_steps.into_iter().for_each(|step| {
        on_cubes = on_cubes
            .iter()
            .map(|cube| match get_cuboid_intersection(&cube, &step.cuboid) {
                Option::Some(intersection) => fracture_cuboid(&cube, &intersection),
                Option::None => vec![*cube],
            })
            .flatten()
            .collect();
        if step.turn_on {
            on_cubes.insert(step.cuboid);
        }
    });

    on_cubes
}

fn get_cuboid_size(cuboid: &Cuboid) -> isize {
    get_range_size(&cuboid.x_range)
        * get_range_size(&cuboid.y_range)
        * get_range_size(&cuboid.z_range)
}

fn get_range_size(range: &Range) -> isize {
    range.high - range.low + 1
}

// breaks this base cuboid into up to 26 individual cubes with the region specified by the sub_cube not represented.
fn fracture_cuboid(base: &Cuboid, sub_cube: &Cuboid) -> Vec<Cuboid> {
    let x_high_range = get_high_range(&base.x_range, &sub_cube.x_range);
    let x_low_range = get_low_range(&base.x_range, &sub_cube.x_range);

    let y_high_range = get_high_range(&base.y_range, &sub_cube.y_range);
    let y_low_range = get_low_range(&base.y_range, &sub_cube.y_range);

    let z_high_range = get_high_range(&base.z_range, &sub_cube.z_range);
    let z_low_range = get_low_range(&base.z_range, &sub_cube.z_range);

    let mut ranges = Vec::new();

    // ==================== top ====================
    // top middle middle
    ranges.push(match z_high_range {
        Option::Some(z_range) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: sub_cube.y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // top middle right
    ranges.push(match (z_high_range, x_high_range) {
        (Option::Some(z_range), Option::Some(x_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: sub_cube.y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // top middle left
    ranges.push(match (z_high_range, x_low_range) {
        (Option::Some(z_range), Option::Some(x_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: sub_cube.y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // top top middle
    ranges.push(match (z_high_range, y_high_range) {
        (Option::Some(z_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // top low middle
    ranges.push(match (z_high_range, y_low_range) {
        (Option::Some(z_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // top top right
    ranges.push(match (z_high_range, y_high_range, x_high_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // top top left
    ranges.push(match (z_high_range, y_high_range, x_low_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // top low right
    ranges.push(match (z_high_range, y_low_range, x_high_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // top low left
    ranges.push(match (z_high_range, y_low_range, x_low_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // ==================== middle ====================
    // middle middle right
    ranges.push(match x_high_range {
        Option::Some(x_range) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: sub_cube.y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle middle left
    ranges.push(match x_low_range {
        Option::Some(x_range) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: sub_cube.y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle top middle
    ranges.push(match y_high_range {
        Option::Some(y_range) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle bottom middle
    ranges.push(match y_low_range {
        Option::Some(y_range) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle top right
    ranges.push(match (x_high_range, y_high_range) {
        (Option::Some(x_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle top left
    ranges.push(match (x_low_range, y_high_range) {
        (Option::Some(x_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle bottom right
    ranges.push(match (x_high_range, y_low_range) {
        (Option::Some(x_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // middle bottom left
    ranges.push(match (x_low_range, y_low_range) {
        (Option::Some(x_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: y_range,
            z_range: sub_cube.z_range,
        }),
        _ => Option::None,
    });

    // ==================== bottom ====================
    // low middle middle
    ranges.push(match z_low_range {
        Option::Some(z_range) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: sub_cube.y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // low middle right
    ranges.push(match (z_low_range, x_high_range) {
        (Option::Some(z_range), Option::Some(x_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: sub_cube.y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // low middle left
    ranges.push(match (z_low_range, x_low_range) {
        (Option::Some(z_range), Option::Some(x_range)) => Option::Some(Cuboid {
            x_range: x_range,
            y_range: sub_cube.y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // low top middle
    ranges.push(match (z_low_range, y_high_range) {
        (Option::Some(z_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // low low middle
    ranges.push(match (z_low_range, y_low_range) {
        (Option::Some(z_range), Option::Some(y_range)) => Option::Some(Cuboid {
            x_range: sub_cube.x_range,
            y_range: y_range,
            z_range: z_range,
        }),
        _ => Option::None,
    });

    // low top right
    ranges.push(match (z_low_range, y_high_range, x_high_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // low top left
    ranges.push(match (z_low_range, y_high_range, x_low_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // low low right
    ranges.push(match (z_low_range, y_low_range, x_high_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    // low low left
    ranges.push(match (z_low_range, y_low_range, x_low_range) {
        (Option::Some(z_range), Option::Some(y_range), Option::Some(x_range)) => {
            Option::Some(Cuboid {
                x_range: x_range,
                y_range: y_range,
                z_range: z_range,
            })
        }
        _ => Option::None,
    });

    ranges
        .into_iter()
        .map(Option::into_iter)
        .flatten()
        .collect()
}

fn get_high_range(base: &Range, sub_range: &Range) -> Option<Range> {
    let low = sub_range.high + 1;
    let high = base.high;
    get_range_from_low_high(low, high)
}

fn get_low_range(base: &Range, sub_range: &Range) -> Option<Range> {
    let high = sub_range.low - 1;
    let low = base.low;
    get_range_from_low_high(low, high)
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
