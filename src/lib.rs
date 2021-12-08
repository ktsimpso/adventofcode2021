#![feature(const_fn_fn_ptr_basics)]

use anyhow::Error;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{character::complete::digit1, combinator::map_res, IResult};
use simple_error::SimpleError;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ops::Sub;

pub enum CommandResult {
    Isize(isize),
    Usize(usize),
}

impl fmt::Debug for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandResult::Isize(val) => val.fmt(f),
            CommandResult::Usize(val) => val.fmt(f),
        }
    }
}

impl From<isize> for CommandResult {
    fn from(item: isize) -> Self {
        CommandResult::Isize(item)
    }
}

impl From<usize> for CommandResult {
    fn from(item: usize) -> Self {
        CommandResult::Usize(item)
    }
}

pub trait Command: Sync {
    fn sub_command(&self) -> App<'static, 'static>;

    fn name(&self) -> &str;

    fn folder_name(&self) -> &str;

    fn run(&self, arguments: &ArgMatches, file: &String) -> Result<CommandResult, Error>;
}

pub struct Problem<'a, A, T> {
    sub_command: fn() -> App<'static, 'static>,
    name: &'a str,
    folder_name: &'a str,
    parse_arguments: fn(&ArgMatches) -> A,
    parse_file: fn(&String) -> IResult<&str, T>,
    run: fn(A, T) -> CommandResult,
}

impl<A, T> Problem<'_, A, T> {
    pub const fn new<'a>(
        sub_command: fn() -> App<'static, 'static>,
        name: &'a str,
        folder_name: &'a str,
        parse_arguments: fn(&ArgMatches) -> A,
        parse_file: fn(&String) -> IResult<&str, T>,
        run: fn(A, T) -> CommandResult,
    ) -> Problem<'a, A, T> {
        Problem {
            sub_command: sub_command,
            name: name,
            folder_name: folder_name,
            parse_arguments: parse_arguments,
            parse_file: parse_file,
            run: run,
        }
    }
}

impl<A, T> Command for Problem<'_, A, T> {
    fn sub_command(&self) -> App<'static, 'static> {
        (self.sub_command)()
    }

    fn name(&self) -> &str {
        self.name
    }

    fn folder_name(&self) -> &str {
        self.folder_name
    }

    fn run(&self, arguments: &ArgMatches, file: &String) -> Result<CommandResult, Error> {
        file_to_string(file)
            .and_then(|file_content| complete_parsing(self.parse_file)(&file_content))
            .map(|t| (self.run)((self.parse_arguments)(arguments), t))
    }
}

pub fn default_sub_command<A, T>(
    command: &Problem<A, T>,
    about: &'static str,
    file_help: &'static str,
    part1_docs: &'static str,
    part2_docs: &'static str,
) -> App<'static, 'static> {
    SubCommand::with_name(command.name())
        .about(about)
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help(file_help)
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about(part1_docs)
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about(part2_docs)
                .version("1.0.0"),
        )
}

pub fn file_to_string(file_name: &String) -> Result<String, Error> {
    File::open(file_name)
        .and_then(|mut file| {
            let mut result = String::new();
            file.read_to_string(&mut result).map(|_| result)
        })
        .map_err(|e| e.into())
}

pub fn complete_parsing<T, U, F>(mut parse_function: F) -> impl FnMut(&T) -> Result<U, Error>
where
    F: FnMut(&T) -> IResult<&str, U>,
{
    move |t| -> Result<U, Error> {
        parse_function(t)
            .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error").into())
            .map(|(_, result)| result)
    }
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usisze_from_string)(input)
}

fn usisze_from_string(input: &str) -> Result<usize, Error> {
    usize::from_str_radix(input, 10).map_err(|err| err.into())
}

pub fn absolute_difference<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if x > y {
        x - y
    } else {
        y - x
    }
}
