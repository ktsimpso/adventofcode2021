use adventofcode2021::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::map_parser,
    multi::{many1, separated_list0},
    IResult,
};
use std::collections::{BTreeSet, HashMap, HashSet};

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
pub struct ChitonArgs {
    expand: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &CHITON,
        "Finds the least risky path from top left to bottom right then sums the risk.",
        "Path to the input file. Input should be newline delimited ranges of integers.",
        "Searches the default input for the path with the lowest risk level.",
        "Expands the default input by 5x then finds the path with the lowest risk level.",
    )    .arg(
        Arg::with_name("expand")
            .short("e")
            .help("The multiplier on the input size. 1 will use the input directly, 5 will expand the input by 5 etc.")
            .takes_value(true)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> ChitonArgs {
    match arguments.subcommand_name() {
        Some("part1") => ChitonArgs { expand: 1usize },
        Some("part2") => ChitonArgs { expand: 5usize },
        _ => ChitonArgs {
            expand: value_t_or_exit!(arguments.value_of("expand"), usize),
        },
    }
}

fn run(arguments: ChitonArgs, cavern: Vec<Vec<usize>>) -> CommandResult {
    let row_max = cavern.len();
    let column_max = cavern.first().unwrap().len();

    let points_to_cost = (0..row_max)
        .map(|y| {
            (0..column_max)
                .map(|x| {
                    (
                        Point { x: x, y: y },
                        *cavern.get(y).unwrap().get(x).unwrap(),
                    )
                })
                .collect()
        })
        .fold(HashMap::new(), |mut acc, row: Vec<(Point, usize)>| {
            acc.extend(row.into_iter());
            acc
        });

    let (points_to_cost, row_max, column_max) =
        expand_points_field(points_to_cost, row_max, column_max, &arguments.expand);

    let mut unvisted_points: HashSet<Point> = points_to_cost.keys().map(|point| *point).collect();

    let mut current = Point {
        x: 0usize,
        y: 0usize,
    };
    let mut costs = HashMap::new();
    let mut unvisited_costs = BTreeSet::new();
    costs.insert(current, 0usize);
    unvisited_costs.insert((0usize, current));

    loop {
        let current_cost = *costs.get(&current).unwrap();
        get_adjacent_points(&(row_max), &(column_max), &current)
            .iter()
            .filter(|point| unvisted_points.contains(point))
            .map(|point| (point, points_to_cost.get(point).unwrap()))
            .for_each(|(point, cost)| {
                let potential_new_cost = current_cost + *cost;
                let new_cost = match costs.get(point) {
                    Some(old_cost) => {
                        unvisited_costs.remove(&(*old_cost, *point));
                        if *old_cost < potential_new_cost {
                            *old_cost
                        } else {
                            potential_new_cost
                        }
                    }
                    None => potential_new_cost,
                };
                costs.insert(*point, new_cost);
                unvisited_costs.insert((new_cost, *point));
            });

        unvisted_points.remove(&current);
        unvisited_costs.remove(&(current_cost, current));

        let result = unvisited_costs.first();

        if let Some((_, next_point)) = result {
            current = *next_point;
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

fn expand_points_field(
    mut points_to_cost: HashMap<Point, usize>,
    mut max_row_size: usize,
    mut max_column_size: usize,
    expand: &usize,
) -> (HashMap<Point, usize>, usize, usize) {
    let mut first_row = vec![points_to_cost];

    for i in 1..*expand {
        let seed = first_row.get(i - 1).unwrap();
        let next = get_next_set_of_points(&max_row_size, &max_column_size, &seed, true);
        first_row.push(next);
    }

    points_to_cost = first_row
        .iter()
        .map(|column| {
            let mut first_column = vec![column.to_owned()];

            for i in 1..*expand {
                let seed = first_column.get(i - 1).unwrap();
                let next = get_next_set_of_points(&max_row_size, &max_column_size, &seed, false);
                first_column.push(next);
            }

            first_column
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<Point, usize>, segment| {
                    acc.extend(segment);
                    acc
                })
        })
        .fold(HashMap::new(), |mut acc, segment| {
            acc.extend(segment);
            acc
        });
    max_row_size *= *expand;
    max_column_size *= *expand;
    (points_to_cost, max_row_size, max_column_size)
}

fn get_next_set_of_points(
    max_row_size: &usize,
    max_column_size: &usize,
    seed: &HashMap<Point, usize>,
    scale_x: bool,
) -> HashMap<Point, usize> {
    seed.iter()
        .map(|(point, cost)| {
            let new_cost = if cost == &9usize {
                1usize
            } else {
                *cost + 1usize
            };

            let new_point = if scale_x {
                Point {
                    x: point.x + max_column_size,
                    y: point.y,
                }
            } else {
                Point {
                    x: point.x,
                    y: point.y + max_row_size,
                }
            };
            (new_point, new_cost)
        })
        .collect()
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
