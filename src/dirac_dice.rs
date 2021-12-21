use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::cmp::min;

pub const DIRAC_DICE: Problem<DiracDiceArgs, (Player, Player)> = Problem::new(
    sub_command,
    "dirac-dice",
    "day21_dirac_dice",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct DiracDiceArgs {}

#[derive(Debug)]
pub struct Player {
    player_number: usize,
    starting_position: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &DIRAC_DICE,
        "Plays a dice game then calculates a value based on the result.",
        "Path to the input file. Input should be newline delimited players and their starting positions.",
        "Plays the game with a detmerinistic die, then multplies the loser's score by the number of rolls.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> DiracDiceArgs {
    match arguments.subcommand_name() {
        Some("part1") => DiracDiceArgs {},
        Some("part2") => DiracDiceArgs {},
        _ => DiracDiceArgs {},
    }
}

fn run(arguments: DiracDiceArgs, players: (Player, Player)) -> CommandResult {
    let (player1, player2) = players;
    let mut die = (1..=100usize).cycle();
    let mut player1_score = 0usize;
    let mut player1_position = player1.starting_position - 1;
    let mut player2_score = 0usize;
    let mut player2_position = player2.starting_position - 1;
    let mut rolls = 0usize;

    loop {
        rolls += 3;
        let next_roll = die.next().expect("infinite iterator")
            + die.next().expect("infinite iterator")
            + die.next().expect("infinite iterator");
        player1_position = (player1_position + next_roll) % 10;
        player1_score += player1_position + 1;

        if player1_score >= 1000 {
            break;
        }

        rolls += 3;
        let next_roll = die.next().expect("infinite iterator")
            + die.next().expect("infinite iterator")
            + die.next().expect("infinite iterator");
        player2_position = (player2_position + next_roll) % 10;
        player2_score += player2_position + 1;

        if player2_score >= 1000 {
            break;
        }
    }

    (min(player1_score, player2_score) * rolls).into()
}

fn parse_data(input: &String) -> IResult<&str, (Player, Player)> {
    tuple((
        terminated(parse_player, newline),
        terminated(parse_player, newline),
    ))(input)
}

fn parse_player(input: &str) -> IResult<&str, Player> {
    map(
        tuple((
            preceded(tag("Player "), parse_usize),
            preceded(tag(" starting position: "), parse_usize),
        )),
        |(player_number, starting_position)| Player {
            player_number: player_number,
            starting_position: starting_position,
        },
    )(input)
}
