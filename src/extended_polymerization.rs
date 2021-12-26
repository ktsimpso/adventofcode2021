use adventofcode2021::{default_sub_command, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, newline},
    combinator::{map, map_parser},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

pub const EXTENDED_POLYMERIZATION: Problem<ExtendedPolymerizationArgs, Polymer<'static>> =
    Problem::new(
        sub_command,
        "extended-polymerization",
        "day14_extended_polymerization",
        parse_arguments,
        parse_data,
        run,
    );

#[derive(Debug)]
pub struct ExtendedPolymerizationArgs {
    polymerization_count: usize,
}

#[derive(Debug)]
pub struct Polymer<'a> {
    template: Vec<&'a str>,
    insertion_rules: HashMap<PolyPair, (PolyPair, PolyPair)>,
}

type PolyPair = (&'static str, &'static str);

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &EXTENDED_POLYMERIZATION,
        "Extends the polymer a certain number of steps then counts the most common character - the least common",
        "Path to the input file. Input should be the start polymer followed by the polymerization rules.",
        "Polymerize the default input 10 times.",
        "Polymerized the default input 40 times.",
    ).arg(
        Arg::with_name("polymerization-count")
            .short("p")
            .help("Number of times to polymerize the template")
            .takes_value(true)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> ExtendedPolymerizationArgs {
    match arguments.subcommand_name() {
        Some("part1") => ExtendedPolymerizationArgs {
            polymerization_count: 10,
        },
        Some("part2") => ExtendedPolymerizationArgs {
            polymerization_count: 40,
        },
        _ => ExtendedPolymerizationArgs {
            polymerization_count: value_t_or_exit!(
                arguments.value_of("polymerization-count"),
                usize
            ),
        },
    }
}

fn run(arguments: ExtendedPolymerizationArgs, polymer: Polymer<'static>) -> CommandResult {
    let mut template = polymer
        .template
        .windows(2)
        .map(|items| (*items.get(0).unwrap(), *items.get(1).unwrap()))
        .fold(HashMap::new(), |mut acc, pair| {
            *acc.entry(pair).or_insert(0usize) += 1;
            acc
        });

    for _ in 0..arguments.polymerization_count {
        template = run_polymer_step(&template, &polymer.insertion_rules);
    }

    let mut counts = template
        .iter()
        .fold(HashMap::new(), |mut acc, ((first, second), count)| {
            *acc.entry(*first).or_insert(0usize) += count;
            *acc.entry(*second).or_insert(0usize) += count;
            acc
        });
    *counts
        .entry(polymer.template.first().unwrap())
        .or_insert(0usize) += 1;
    *counts
        .entry(polymer.template.last().unwrap())
        .or_insert(0usize) += 1;
    counts = counts
        .iter()
        .map(|(key, value)| (*key, value / 2))
        .collect();

    let top = counts.iter().map(|(_, count)| count).max().unwrap();
    let bottom = counts.iter().map(|(_, count)| count).min().unwrap();

    (top - bottom).into()
}

fn run_polymer_step(
    template: &HashMap<PolyPair, usize>,
    insertion_rules: &HashMap<PolyPair, (PolyPair, PolyPair)>,
) -> HashMap<PolyPair, usize> {
    template
        .iter()
        .fold(HashMap::new(), |mut acc, (pair, count)| {
            let (new1, new2) = insertion_rules.get(pair).unwrap();
            *acc.entry(*new1).or_insert(0usize) += count;
            *acc.entry(*new2).or_insert(0usize) += count;
            acc
        })
}

fn parse_data(input: &String) -> IResult<&str, Polymer<'static>> {
    map(
        separated_pair(parse_polymer_template, tag("\n\n"), parse_insertion_rules),
        |(template, insertion_rules)| Polymer {
            template: template,
            insertion_rules: insertion_rules,
        },
    )(Box::leak(input.clone().into_boxed_str()))
}

fn parse_polymer_template(input: &str) -> IResult<&str, Vec<&str>> {
    many1(map_parser(take(1usize), alpha1))(input)
}

fn parse_insertion_rules(
    input: &'static str,
) -> IResult<&str, HashMap<PolyPair, (PolyPair, PolyPair)>> {
    map(
        separated_list1(
            newline,
            separated_pair(parse_polymer_template, tag(" -> "), alpha1),
        ),
        |insertion_rules| {
            insertion_rules
                .into_iter()
                .fold(HashMap::new(), |mut acc, (key, value)| {
                    let left = (*key.get(0).unwrap(), value);
                    let right = (value, *key.get(1).unwrap());
                    let new_key = (*key.get(0).unwrap(), *key.get(1).unwrap());
                    acc.insert(new_key, (left, right));
                    acc
                })
        },
    )(input)
}
