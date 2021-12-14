use std::collections::HashMap;

use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, newline},
    combinator::{map, map_parser},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

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
pub struct ExtendedPolymerizationArgs {}

#[derive(Debug)]
pub struct Polymer<'a> {
    template: Vec<&'a str>,
    insertion_rules: HashMap<Vec<&'a str>, &'a str>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &EXTENDED_POLYMERIZATION,
        "Extends the polymer a certain number of steps then counts the most common character - the least common",
        "Path to the input file. Input should be the start polymer followed by the polymerization rules.",
        "Polymerized the default input 10 times.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> ExtendedPolymerizationArgs {
    match arguments.subcommand_name() {
        Some("part1") => ExtendedPolymerizationArgs {},
        Some("part2") => ExtendedPolymerizationArgs {},
        _ => ExtendedPolymerizationArgs {},
    }
}

fn run(arguments: ExtendedPolymerizationArgs, mut polymer: Polymer<'static>) -> CommandResult {
    for _ in 0..10 {
        polymer = run_polymer_step(&polymer);
    }

    let counts = polymer
        .template
        .iter()
        .fold(HashMap::new(), |mut acc, value| {
            *acc.entry(value).or_insert(0usize) += 1;
            acc
        });
    let top = counts.iter().map(|(_, count)| count).max().unwrap();
    let bottom = counts.iter().map(|(_, count)| count).min().unwrap();

    (top - bottom).into()
}

fn run_polymer_step(polymer: &Polymer<'static>) -> Polymer<'static> {
    let new_template = polymer
        .template
        .windows(2usize)
        .map(|pair| {
            let mut pair_list = pair.to_vec();
            if polymer.insertion_rules.contains_key(&pair_list) {
                pair_list.insert(1, polymer.insertion_rules.get(&pair_list).unwrap());
                pair_list
            } else {
                pair_list
            }
        })
        .reduce(|mut acc, mut next| {
            next.remove(0);
            acc.append(&mut next);
            acc
        })
        .unwrap();

    Polymer {
        template: new_template,
        insertion_rules: polymer.insertion_rules.clone(),
    }
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

fn parse_insertion_rules(input: &str) -> IResult<&str, HashMap<Vec<&str>, &str>> {
    map(
        separated_list1(
            newline,
            separated_pair(parse_polymer_template, tag(" -> "), alpha1),
        ),
        |insertion_rules| {
            insertion_rules
                .into_iter()
                .fold(HashMap::new(), |mut acc, (key, value)| {
                    acc.insert(key, value);
                    acc
                })
        },
    )(input)
}
