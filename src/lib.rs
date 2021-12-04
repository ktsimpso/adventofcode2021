#![feature(const_fn_fn_ptr_basics)]

use anyhow::Error;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{character::complete::digit1, combinator::map_res, IResult};
use simple_error::SimpleError;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
pub struct Command<'a> {
    sub_command: fn() -> App<'static, 'static>,
    name: &'a str,
    folder_name: &'a str,
    run: fn(&ArgMatches, &String) -> Result<CommandResult, Error>,
}

impl Command<'_> {
    pub const fn new<'a>(
        sub_command: fn() -> App<'static, 'static>,
        name: &'a str,
        folder_name: &'a str,
        run: fn(&ArgMatches, &String) -> Result<CommandResult, Error>,
    ) -> Command<'a> {
        Command {
            sub_command: sub_command,
            name: name,
            folder_name: folder_name,
            run: run,
        }
    }

    pub fn sub_command(&self) -> App<'static, 'static> {
        (self.sub_command)()
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn folder_name(&self) -> &str {
        self.folder_name
    }

    pub fn run(&self, arguments: &ArgMatches, file: &String) -> Result<CommandResult, Error> {
        (self.run)(arguments, file)
    }
}

pub fn default_sub_command(
    command: &Command,
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

pub fn file_to_lines(file_name: &String) -> Result<Vec<String>, Error> {
    File::open(file_name)
        .map_err(|err| err.into())
        .and_then(|file| {
            BufReader::new(file)
                .lines()
                .try_fold(Vec::new(), |mut lines, line_result| {
                    line_result.map(|line| {
                        lines.push(line);
                        lines
                    })
                })
                .map_err(|err| err.into())
        })
}

pub fn file_to_string(file_name: &String) -> Result<String, Error> {
    file_to_lines(file_name).map(|lines| {
        lines.into_iter().fold(String::new(), |mut acc, line| {
            acc.push_str(&line.to_string());
            acc.push('\n');
            acc
        })
    })
}

pub fn parse_lines<T, U, E, F>(lines: Vec<T>, mut parse_function: F) -> Result<Vec<U>, E>
where
    F: FnMut(&T) -> Result<U, E>,
{
    lines
        .into_iter()
        .try_fold(Vec::new(), |mut parsed_lines, line| {
            parse_function(&line).map(|parsed_line| {
                parsed_lines.push(parsed_line);
                parsed_lines
            })
        })
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
