use adventofcode2021::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::{
    cmp::{max, min},
    collections::HashMap,
};
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const DIRAC_DICE: Problem<DiracDiceArgs, (Player, Player)> = Problem::new(
    sub_command,
    "dirac-dice",
    "day21_dirac_dice",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct DiracDiceArgs {
    game_type: GameType,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum GameType {
    Deterministic,
    Dirac,
}

#[derive(Debug)]
pub struct Player {
    starting_position: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PlayerUniverse {
    player1_position: usize,
    player2_position: usize,
    player1_score: usize,
    player2_score: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &DIRAC_DICE,
        "Plays a dice game then calculates a value based on the result.",
        "Path to the input file. Input should be newline delimited players and their starting positions.",
        "Plays the game with the default input with a detmerinistic die, then multplies the loser's score by the number of rolls.",
        "Plays the game with the default input with dirac dice. Then outputs the total number of universes where the player that won the most won.",
    ).arg(
        Arg::with_name("game-type")
            .short("g")
            .help(
                "The type of game to play. The games available are as follows:\n\n\
            deterministic: Uses a d100 which always rolls one higher.\n\n\
            dirac: Uses a d3 dirac die and finds the results for all universes.\n\n",
            )
            .takes_value(true)
            .possible_values(&GameType::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> DiracDiceArgs {
    match arguments.subcommand_name() {
        Some("part1") => DiracDiceArgs {
            game_type: GameType::Deterministic,
        },
        Some("part2") => DiracDiceArgs {
            game_type: GameType::Dirac,
        },
        _ => DiracDiceArgs {
            game_type: value_t_or_exit!(arguments.value_of("game-type"), GameType),
        },
    }
}

fn run(arguments: DiracDiceArgs, players: (Player, Player)) -> CommandResult {
    match arguments.game_type {
        GameType::Deterministic => play_deterministic_game(players),
        GameType::Dirac => play_dirac_games(players),
    }
    .into()
}

fn play_dirac_games(players: (Player, Player)) -> usize {
    let (player1, player2) = players;

    let die_outcomes = vec![
        (3usize, 1usize),
        (4usize, 3usize),
        (5usize, 6usize),
        (6usize, 7usize),
        (7usize, 6usize),
        (8usize, 3usize),
        (9usize, 1usize),
    ];

    let mut games = HashMap::new();
    games.insert(
        PlayerUniverse {
            player1_position: player1.starting_position - 1,
            player2_position: player2.starting_position - 1,
            player1_score: 0usize,
            player2_score: 0usize,
        },
        1usize,
    );

    let mut player1_wins = 0usize;
    let mut player2_wins = 0usize;

    while games.len() > 0 {
        games = games
            .iter()
            .map(|(game, count)| {
                die_outcomes
                    .iter()
                    .map(|(die_roll, die_count)| {
                        let player1_position = (game.player1_position + die_roll) % 10;
                        let player1_score = game.player1_score + player1_position + 1;
                        (
                            PlayerUniverse {
                                player1_position: player1_position,
                                player2_position: game.player2_position,
                                player1_score: player1_score,
                                player2_score: game.player2_score,
                            },
                            count * die_count,
                        )
                    })
                    .collect()
            })
            .fold(
                HashMap::new(),
                |mut acc, results: Vec<(PlayerUniverse, usize)>| {
                    results
                        .iter()
                        .for_each(|(game, count)| *acc.entry(*game).or_insert(0usize) += *count);

                    acc
                },
            );

        let winning_games: HashMap<PlayerUniverse, usize> = games
            .iter()
            .filter(|(game, _)| game.player1_score >= 21)
            .map(|(game, count)| (*game, *count))
            .collect();
        winning_games.iter().for_each(|(game, count)| {
            games.remove(game);
            player1_wins += count;
        });

        games = games
            .iter()
            .map(|(game, count)| {
                die_outcomes
                    .iter()
                    .map(|(die_roll, die_count)| {
                        let player2_position = (game.player2_position + die_roll) % 10;
                        let player2_score = game.player2_score + player2_position + 1;
                        (
                            PlayerUniverse {
                                player1_position: game.player1_position,
                                player2_position: player2_position,
                                player1_score: game.player1_score,
                                player2_score: player2_score,
                            },
                            count * die_count,
                        )
                    })
                    .collect()
            })
            .fold(
                HashMap::new(),
                |mut acc, results: Vec<(PlayerUniverse, usize)>| {
                    results
                        .iter()
                        .for_each(|(game, count)| *acc.entry(*game).or_insert(0usize) += *count);

                    acc
                },
            );

        let winning_games: HashMap<PlayerUniverse, usize> = games
            .iter()
            .filter(|(game, _)| game.player2_score >= 21)
            .map(|(game, count)| (*game, *count))
            .collect();
        winning_games.iter().for_each(|(game, count)| {
            games.remove(game);
            player2_wins += count;
        });
    }

    max(player1_wins, player2_wins)
}

fn play_deterministic_game(players: (Player, Player)) -> usize {
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
        |(_, starting_position)| Player {
            starting_position: starting_position,
        },
    )(input)
}
