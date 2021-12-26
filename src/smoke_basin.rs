use adventofcode2021::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::map_parser,
    multi::{many1, separated_list0},
    IResult,
};
use std::collections::HashSet;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const SMOKE_BASIN: Problem<SmokeBasinArgs, Vec<Vec<usize>>> = Problem::new(
    sub_command,
    "smoke-basin",
    "day9_smoke_basin",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct SmokeBasinArgs {
    topography_function: TopographyFunction,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum TopographyFunction {
    RiskLevel,
    BigBasins,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SMOKE_BASIN,
        "Finds low points in lava tube smoke stacks then calculates values",
        "Path to the input file. Input should be integers and newlines with equal sizes.",
        "Searches the default input for the total risk level.",
        "Searches the default input for the largest three basins, then multiplies their sizes.",
    )
    .arg(
        Arg::with_name("topography-function")
            .short("t")
            .help(
                "The type topography requests. The functions available are as follows:\n\n\
            risk-level: Finds the low points then calculates the total risk level.\n\n\
            big-basin: Finds the largest three basins then multiplies thier sizes.\n\n",
            )
            .takes_value(true)
            .possible_values(&TopographyFunction::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> SmokeBasinArgs {
    match arguments.subcommand_name() {
        Some("part1") => SmokeBasinArgs {
            topography_function: TopographyFunction::RiskLevel,
        },
        Some("part2") => SmokeBasinArgs {
            topography_function: TopographyFunction::BigBasins,
        },
        _ => SmokeBasinArgs {
            topography_function: value_t_or_exit!(
                arguments.value_of("topography-function"),
                TopographyFunction
            ),
        },
    }
}

fn run(arguments: SmokeBasinArgs, smoke_points: Vec<Vec<usize>>) -> CommandResult {
    let topography = match arguments.topography_function {
        TopographyFunction::RiskLevel => calculate_risk_level,
        TopographyFunction::BigBasins => calculate_top_3_basin_sizes,
    };

    let low_points = find_low_points(&smoke_points);
    topography(&low_points, &smoke_points).into()
}

fn calculate_risk_level(low_points: &Vec<(usize, usize)>, smoke_points: &Vec<Vec<usize>>) -> usize {
    low_points
        .iter()
        .map(|(i, j)| smoke_points.get(*i).unwrap().get(*j).unwrap() + 1)
        .fold(0usize, |sum, risk_level| sum + risk_level)
}

fn calculate_top_3_basin_sizes(
    low_points: &Vec<(usize, usize)>,
    smoke_points: &Vec<Vec<usize>>,
) -> usize {
    let column_length = smoke_points.len();
    let row_length = smoke_points.first().unwrap().len();
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|low_point| {
            let basin = &mut HashSet::new();
            find_basin_from_low_point(
                *low_point,
                &smoke_points,
                &column_length,
                &row_length,
                basin,
            );
            basin.len()
        })
        .collect();

    basin_sizes.sort();

    basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .fold(1usize, |product, basin_size| product * basin_size)
        .into()
}

fn find_basin_from_low_point(
    low_point: (usize, usize),
    smoke_points: &Vec<Vec<usize>>,
    column_length: &usize,
    row_length: &usize,
    result: &mut HashSet<(usize, usize)>,
) -> () {
    let (mut x, mut y) = low_point;
    result.insert(low_point);

    if x > 0usize {
        x -= 1usize;
        if *smoke_points.get(x).unwrap().get(y).unwrap() < 9usize && !result.contains(&(x, y)) {
            find_basin_from_low_point((x, y), &smoke_points, column_length, row_length, result);
        }
    }

    x = low_point.0;

    if x < (*column_length - 1usize) {
        x += 1usize;
        if *smoke_points.get(x).unwrap().get(y).unwrap() < 9usize && !result.contains(&(x, y)) {
            find_basin_from_low_point((x, y), &smoke_points, column_length, row_length, result);
        }
    }

    x = low_point.0;

    if y > 0usize {
        y -= 1usize;
        if *smoke_points.get(x).unwrap().get(y).unwrap() < 9usize && !result.contains(&(x, y)) {
            find_basin_from_low_point((x, y), &smoke_points, column_length, row_length, result);
        }
    }

    y = low_point.1;

    if y < (*row_length - 1usize) {
        y += 1usize;
        if *smoke_points.get(x).unwrap().get(y).unwrap() < 9usize && !result.contains(&(x, y)) {
            find_basin_from_low_point((x, y), &smoke_points, column_length, row_length, result);
        }
    }
}

fn find_low_points(smoke_points: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let column_length = smoke_points.len();
    let mut low_points = Vec::new();
    for i in 0..column_length {
        let row = smoke_points.get(i).unwrap();
        let row_length = row.len();
        for j in 0..row_length {
            let current = row.get(j).unwrap();
            let adjacents = get_adjacent_indicies((&i, &j), &column_length, &row_length);
            let low_point = adjacents
                .iter()
                .map(|(x, y)| smoke_points.get(*x).unwrap().get(*y).unwrap())
                .all(|value| current < value);

            if low_point {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn get_adjacent_indicies(
    point: (&usize, &usize),
    column_length: &usize,
    row_length: &usize,
) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let mut adjacents = Vec::new();
    if *y > 0usize {
        adjacents.push((*x, y - 1));
    }

    if *y < (*row_length - 1usize) {
        adjacents.push((*x, y + 1));
    }

    if *x > 0usize {
        adjacents.push((x - 1, *y));
    }

    if *x < (*column_length - 1usize) {
        adjacents.push((x + 1, *y));
    }

    adjacents
}

fn parse_data(input: &String) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    many1(map_parser(take(1usize), parse_usize))(input)
}
