use adventofcode2021::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::map_parser,
    multi::{many0, separated_list0},
    IResult,
};
use std::collections::HashSet;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const DUMBO_OCTOPUS: Problem<DumboOctopusArgs, Vec<Vec<usize>>> = Problem::new(
    sub_command,
    "dumbo-octopus",
    "day11_dumbo_octopus",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct DumboOctopusArgs {
    simulation_parameters: SimulationParameters,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum SimulationParameters {
    OneHundredSteps,
    SynchronizedFlashes,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &DUMBO_OCTOPUS,
        "Simulates dumbo octopi flashing behavoir.",
        "Path to the input file. Input should be newline delimited sets of 10 integers from 0-9.",
        "Counts the number of flashes after 100 steps.",
        "Counts the number of iterartions before all octopi flash.",
    ).arg(
        Arg::with_name("simulation-parameters")
            .short("s")
            .help(
                "How long the octopi should be simulated for. The functions available are as follows:\n\n\
            one-hundred-steps: Counts the number of flashes that happen after 100 steps.\n\n\
            synchronized-flashes: Counts the number of steps needed before all octopi flash at once.\n\n",
            )
            .takes_value(true)
            .possible_values(&SimulationParameters::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> DumboOctopusArgs {
    match arguments.subcommand_name() {
        Some("part1") => DumboOctopusArgs {
            simulation_parameters: SimulationParameters::OneHundredSteps,
        },
        Some("part2") => DumboOctopusArgs {
            simulation_parameters: SimulationParameters::SynchronizedFlashes,
        },
        _ => DumboOctopusArgs {
            simulation_parameters: value_t_or_exit!(
                arguments.value_of("simulation-parameters"),
                SimulationParameters
            ),
        },
    }
}

fn run(arguments: DumboOctopusArgs, octopi: Vec<Vec<usize>>) -> CommandResult {
    match arguments.simulation_parameters {
        SimulationParameters::OneHundredSteps => count_flashes_after_100_steps(octopi),
        SimulationParameters::SynchronizedFlashes => first_iteration_where_all_flash(octopi),
    }
    .into()
}

fn first_iteration_where_all_flash(mut octopi: Vec<Vec<usize>>) -> usize {
    let mut i = 0usize;
    loop {
        i += 1;
        let (new_octopi, _) = run_step(&octopi);
        octopi = new_octopi;
        if octopi
            .iter()
            .all(|row| row.iter().all(|octopus| *octopus == 0usize))
        {
            break;
        }
    }
    i.into()
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
