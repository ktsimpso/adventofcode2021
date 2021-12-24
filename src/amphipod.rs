use std::collections::HashSet;

use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

pub const AMPHIPOD: Problem<AmphipodArgs, (Vec<Amphipod>, Vec<Amphipod>)> = Problem::new(
    sub_command,
    "amphipod",
    "day23_amphipod",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct AmphipodArgs {
    additional_rows: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn multiplier(&self) -> usize {
        match self {
            Amphipod::Amber => 1usize,
            Amphipod::Bronze => 10usize,
            Amphipod::Copper => 100usize,
            Amphipod::Desert => 1000usize,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AmphipodGame {
    block_depth: usize,
    energy: usize,
    far_left_buffer: Option<Amphipod>,
    left_buffer: Option<Amphipod>,
    ab_buffer: Option<Amphipod>,
    bc_buffer: Option<Amphipod>,
    cd_buffer: Option<Amphipod>,
    right_buffer: Option<Amphipod>,
    far_right_buffer: Option<Amphipod>,
    a_block: Vec<Amphipod>,
    b_block: Vec<Amphipod>,
    c_block: Vec<Amphipod>,
    d_block: Vec<Amphipod>,
}

impl AmphipodGame {
    fn is_a_block_valid(&self) -> bool {
        self.a_block.iter().all(|amphipod| match amphipod {
            Amphipod::Amber => true,
            _ => false,
        })
    }

    fn is_b_block_valid(&self) -> bool {
        self.b_block.iter().all(|amphipod| match amphipod {
            Amphipod::Bronze => true,
            _ => false,
        })
    }

    fn is_c_block_valid(&self) -> bool {
        self.c_block.iter().all(|amphipod| match amphipod {
            Amphipod::Copper => true,
            _ => false,
        })
    }

    fn is_d_block_valid(&self) -> bool {
        self.d_block.iter().all(|amphipod| match amphipod {
            Amphipod::Desert => true,
            _ => false,
        })
    }
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &AMPHIPOD,
        "Finds solutions to the amphipod game, returns the solution value with the lowest energy.",
        "Path to the input file. Input should be the intial setup of the game.",
        "Finds the result of the gane for the default input.",
        "Finds the result of the gane for the default input with additional rows added.",
    )
    .arg(
        Arg::with_name("additional-rows")
            .short("a")
            .help("If passed, adds two more rows to the amphipod game."),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> AmphipodArgs {
    match arguments.subcommand_name() {
        Some("part1") => AmphipodArgs {
            additional_rows: false,
        },
        Some("part2") => AmphipodArgs {
            additional_rows: true,
        },
        _ => AmphipodArgs {
            additional_rows: arguments.is_present("additional-rows"),
        },
    }
}

fn run(
    arguments: AmphipodArgs,
    starting_positions: (Vec<Amphipod>, Vec<Amphipod>),
) -> CommandResult {
    let (top, bottom) = starting_positions;
    let a_block = if arguments.additional_rows {
        vec![bottom[0], Amphipod::Desert, Amphipod::Desert, top[0]]
    } else {
        vec![bottom[0], top[0]]
    };
    let b_block = if arguments.additional_rows {
        vec![bottom[1], Amphipod::Bronze, Amphipod::Copper, top[1]]
    } else {
        vec![bottom[1], top[1]]
    };
    let c_block = if arguments.additional_rows {
        vec![bottom[2], Amphipod::Amber, Amphipod::Bronze, top[2]]
    } else {
        vec![bottom[2], top[2]]
    };
    let d_block = if arguments.additional_rows {
        vec![bottom[3], Amphipod::Copper, Amphipod::Amber, top[3]]
    } else {
        vec![bottom[3], top[3]]
    };

    let game = AmphipodGame {
        block_depth: a_block.len(),
        energy: 0,
        far_left_buffer: Option::None,
        left_buffer: Option::None,
        ab_buffer: Option::None,
        bc_buffer: Option::None,
        cd_buffer: Option::None,
        right_buffer: Option::None,
        far_right_buffer: Option::None,
        a_block: a_block,
        b_block: b_block,
        c_block: c_block,
        d_block: d_block,
    };

    let mut games = HashSet::from([game]);
    let mut winning_games = HashSet::new();

    while games.len() > 0 {
        games = games
            .into_iter()
            .filter_map(|game| {
                let moves = get_all_valid_moves(&game);
                if moves.len() > 0 {
                    Option::Some(moves)
                } else {
                    Option::None
                }
            })
            .flatten()
            .filter(|game| {
                let winner = is_game_winner(&game);

                if winner {
                    winning_games.insert(game.clone());
                }

                !winner
            })
            .collect();
    }

    winning_games
        .into_iter()
        .map(|game| game.energy)
        .min()
        .expect("At least one winner")
        .into()
}

fn is_game_winner(game: &AmphipodGame) -> bool {
    let a_valid = game.is_a_block_valid();
    let b_valid = game.is_b_block_valid();
    let c_valid = game.is_c_block_valid();
    let d_valid = game.is_d_block_valid();

    a_valid
        && b_valid
        && c_valid
        && d_valid
        && game.far_left_buffer.is_none()
        && game.left_buffer.is_none()
        && game.ab_buffer.is_none()
        && game.bc_buffer.is_none()
        && game.cd_buffer.is_none()
        && game.right_buffer.is_none()
        && game.far_right_buffer.is_none()
}

fn get_all_valid_moves(game: &AmphipodGame) -> Vec<AmphipodGame> {
    let mut valid_moves = a_block_valid_moves(&game);
    valid_moves.extend(b_block_valid_moves(&game));
    valid_moves.extend(c_block_valid_moves(&game));
    valid_moves.extend(d_block_valid_moves(&game));
    valid_moves.extend(far_left_buffer_valid_moves(&game));
    valid_moves.extend(left_buffer_valid_moves(&game));
    valid_moves.extend(ab_buffer_valid_moves(&game));
    valid_moves.extend(bc_buffer_valid_moves(&game));
    valid_moves.extend(cd_buffer_valid_moves(&game));
    valid_moves.extend(right_buffer_valid_moves(&game));
    valid_moves.extend(far_right_buffer_valid_moves(&game));

    valid_moves
}

fn a_block_valid_moves(game: &AmphipodGame) -> Vec<AmphipodGame> {
    if game.is_a_block_valid() {
        return Vec::new();
    }

    let mut moves = Vec::new();
    let amphipod = game
        .a_block
        .last()
        .expect("Empty a block would trigger valid check");
    let base_cost = game.block_depth - game.a_block.len();

    match (game.far_left_buffer, game.left_buffer) {
        (Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_left_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (3 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.left_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, Option::None) => {
            let mut new_game = game.clone();
            new_game.left_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    };

    match (
        game.ab_buffer,
        game.bc_buffer,
        game.cd_buffer,
        game.right_buffer,
        game.far_right_buffer,
    ) {
        (Option::None, Option::None, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_right_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (9 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.right_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (8 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, Option::None, Option::None, Option::None, _) => {
            let mut new_game = game.clone();
            new_game.right_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (8 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, Option::None, Option::None, _, _) => {
            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, Option::None, _, _, _) => {
            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, _, _, _, _) => {
            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.a_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    }

    moves
}

fn b_block_valid_moves(game: &AmphipodGame) -> Vec<AmphipodGame> {
    if game.is_b_block_valid() {
        return Vec::new();
    }

    let mut moves = Vec::new();
    let amphipod = game
        .b_block
        .last()
        .expect("Empty a block would trigger valid check");
    let base_cost = game.block_depth - game.b_block.len();

    match (game.far_left_buffer, game.left_buffer, game.ab_buffer) {
        (Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_left_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (5 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.left_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.left_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, _, Option::None) => {
            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    };

    match (
        game.bc_buffer,
        game.cd_buffer,
        game.right_buffer,
        game.far_right_buffer,
    ) {
        (Option::None, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_right_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (7 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.right_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, Option::None, Option::None, _) => {
            let mut new_game = game.clone();
            new_game.right_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, Option::None, _, _) => {
            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, _, _, _) => {
            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.b_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    }

    moves
}

fn c_block_valid_moves(game: &AmphipodGame) -> Vec<AmphipodGame> {
    if game.is_c_block_valid() {
        return Vec::new();
    }

    let mut moves = Vec::new();
    let amphipod = game
        .c_block
        .last()
        .expect("Empty a block would trigger valid check");
    let base_cost = game.block_depth - game.c_block.len();

    match (
        game.far_left_buffer,
        game.left_buffer,
        game.ab_buffer,
        game.bc_buffer,
    ) {
        (Option::None, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_left_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (7 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.left_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.left_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, _, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, _, _, Option::None) => {
            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    };

    match (game.cd_buffer, game.right_buffer, game.far_right_buffer) {
        (Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_right_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (5 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.right_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, Option::None, _) => {
            let mut new_game = game.clone();
            new_game.right_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, _, _) => {
            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.c_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    }

    moves
}

fn d_block_valid_moves(game: &AmphipodGame) -> Vec<AmphipodGame> {
    if game.is_d_block_valid() {
        return Vec::new();
    }

    let mut moves = Vec::new();
    let amphipod = game
        .d_block
        .last()
        .expect("Empty a block would trigger valid check");
    let base_cost = game.block_depth - game.d_block.len();

    match (
        game.far_left_buffer,
        game.left_buffer,
        game.ab_buffer,
        game.bc_buffer,
        game.cd_buffer,
    ) {
        (Option::None, Option::None, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_left_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (9 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.left_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (8 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, Option::None, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.left_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (8 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, _, Option::None, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.ab_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (6 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, _, _, Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.bc_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (4 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (_, _, _, _, Option::None) => {
            let mut new_game = game.clone();
            new_game.cd_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    };

    match (game.right_buffer, game.far_right_buffer) {
        (Option::None, Option::None) => {
            let mut new_game = game.clone();
            new_game.far_right_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (3 + base_cost);
            moves.push(new_game);

            let mut new_game = game.clone();
            new_game.right_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        (Option::None, _) => {
            let mut new_game = game.clone();
            new_game.right_buffer = new_game.d_block.pop();
            new_game.energy += amphipod.multiplier() * (2 + base_cost);
            moves.push(new_game);
        }
        _ => (),
    }

    moves
}

fn far_left_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.far_left_buffer
        .iter()
        .find_map(|amphipod| match amphipod {
            Amphipod::Amber => match (game.is_a_block_valid(), game.left_buffer) {
                (true, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.far_left_buffer = Option::None;
                    next_step.a_block.push(Amphipod::Amber);
                    next_step.energy += 3 + amphipod.multiplier()
                        * (next_step.block_depth - next_step.a_block.len());
                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Bronze => match (game.is_b_block_valid(), game.left_buffer, game.ab_buffer) {
                (true, Option::None, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.far_left_buffer = Option::None;
                    next_step.b_block.push(Amphipod::Bronze);
                    next_step.energy += 50
                        + amphipod.multiplier() * (next_step.block_depth - next_step.b_block.len());

                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Copper => {
                match (
                    game.is_c_block_valid(),
                    game.left_buffer,
                    game.ab_buffer,
                    game.bc_buffer,
                ) {
                    (true, Option::None, Option::None, Option::None) => {
                        let mut next_step = game.clone();
                        next_step.far_left_buffer = Option::None;
                        next_step.c_block.push(Amphipod::Copper);
                        next_step.energy += 700
                            + amphipod.multiplier()
                                * (next_step.block_depth - next_step.c_block.len());

                        Option::Some(next_step)
                    }
                    _ => Option::None,
                }
            }
            Amphipod::Desert => {
                match (
                    game.is_d_block_valid(),
                    game.left_buffer,
                    game.ab_buffer,
                    game.bc_buffer,
                    game.cd_buffer,
                ) {
                    (true, Option::None, Option::None, Option::None, Option::None) => {
                        let mut next_step = game.clone();
                        next_step.far_left_buffer = Option::None;
                        next_step.d_block.push(Amphipod::Desert);
                        next_step.energy += 9000
                            + amphipod.multiplier()
                                * (next_step.block_depth - next_step.d_block.len());
                        Option::Some(next_step)
                    }
                    _ => Option::None,
                }
            }
        })
}

fn left_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.left_buffer.iter().find_map(|amphipod| match amphipod {
        Amphipod::Amber => match game.is_a_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.left_buffer = Option::None;
                next_step.a_block.push(Amphipod::Amber);
                next_step.energy +=
                    2 + amphipod.multiplier() * (next_step.block_depth - next_step.a_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Bronze => match (game.is_b_block_valid(), game.ab_buffer) {
            (true, Option::None) => {
                let mut next_step = game.clone();
                next_step.left_buffer = Option::None;
                next_step.b_block.push(Amphipod::Bronze);
                next_step.energy +=
                    40 + amphipod.multiplier() * (next_step.block_depth - next_step.b_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Copper => match (game.is_c_block_valid(), game.ab_buffer, game.bc_buffer) {
            (true, Option::None, Option::None) => {
                let mut next_step = game.clone();
                next_step.left_buffer = Option::None;
                next_step.c_block.push(Amphipod::Copper);
                next_step.energy +=
                    600 + amphipod.multiplier() * (next_step.block_depth - next_step.c_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Desert => {
            match (
                game.is_d_block_valid(),
                game.ab_buffer,
                game.bc_buffer,
                game.cd_buffer,
            ) {
                (true, Option::None, Option::None, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.left_buffer = Option::None;
                    next_step.d_block.push(Amphipod::Desert);
                    next_step.energy += 8000
                        + amphipod.multiplier() * (next_step.block_depth - next_step.d_block.len());
                    Option::Some(next_step)
                }
                _ => Option::None,
            }
        }
    })
}

fn ab_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.ab_buffer.iter().find_map(|amphipod| match amphipod {
        Amphipod::Amber => match game.is_a_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.ab_buffer = Option::None;
                next_step.a_block.push(Amphipod::Amber);
                next_step.energy +=
                    2 + amphipod.multiplier() * (next_step.block_depth - next_step.a_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Bronze => match game.is_b_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.ab_buffer = Option::None;
                next_step.b_block.push(Amphipod::Bronze);
                next_step.energy +=
                    20 + amphipod.multiplier() * (next_step.block_depth - next_step.b_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Copper => match (game.is_c_block_valid(), game.bc_buffer) {
            (true, Option::None) => {
                let mut next_step = game.clone();
                next_step.ab_buffer = Option::None;
                next_step.c_block.push(Amphipod::Copper);
                next_step.energy +=
                    400 + amphipod.multiplier() * (next_step.block_depth - next_step.c_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Desert => match (game.is_d_block_valid(), game.bc_buffer, game.cd_buffer) {
            (true, Option::None, Option::None) => {
                let mut next_step = game.clone();
                next_step.ab_buffer = Option::None;
                next_step.d_block.push(Amphipod::Desert);
                next_step.energy += 6000
                    + amphipod.multiplier() * (next_step.block_depth - next_step.d_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
    })
}

fn bc_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.bc_buffer.iter().find_map(|amphipod| match amphipod {
        Amphipod::Amber => match (game.is_a_block_valid(), game.ab_buffer) {
            (true, Option::None) => {
                let mut next_step = game.clone();
                next_step.bc_buffer = Option::None;
                next_step.a_block.push(Amphipod::Amber);
                next_step.energy +=
                    4 + amphipod.multiplier() * (next_step.block_depth - next_step.a_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Bronze => match game.is_b_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.bc_buffer = Option::None;
                next_step.b_block.push(Amphipod::Bronze);
                next_step.energy +=
                    20 + amphipod.multiplier() * (next_step.block_depth - next_step.b_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Copper => match game.is_c_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.bc_buffer = Option::None;
                next_step.c_block.push(Amphipod::Copper);
                next_step.energy +=
                    200 + amphipod.multiplier() * (next_step.block_depth - next_step.c_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Desert => match (game.is_d_block_valid(), game.cd_buffer) {
            (true, Option::None) => {
                let mut next_step = game.clone();
                next_step.bc_buffer = Option::None;
                next_step.d_block.push(Amphipod::Desert);
                next_step.energy += 4000
                    + amphipod.multiplier() * (next_step.block_depth - next_step.d_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
    })
}

fn cd_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.cd_buffer.iter().find_map(|amphipod| match amphipod {
        Amphipod::Amber => match (game.is_a_block_valid(), game.ab_buffer, game.bc_buffer) {
            (true, Option::None, Option::None) => {
                let mut next_step = game.clone();
                next_step.cd_buffer = Option::None;
                next_step.a_block.push(Amphipod::Amber);
                next_step.energy +=
                    6 + amphipod.multiplier() * (next_step.block_depth - next_step.a_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Bronze => match (game.is_b_block_valid(), game.bc_buffer) {
            (true, Option::None) => {
                let mut next_step = game.clone();
                next_step.cd_buffer = Option::None;
                next_step.b_block.push(Amphipod::Bronze);
                next_step.energy +=
                    40 + amphipod.multiplier() * (next_step.block_depth - next_step.b_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Copper => match game.is_c_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.cd_buffer = Option::None;
                next_step.c_block.push(Amphipod::Copper);
                next_step.energy +=
                    200 + amphipod.multiplier() * (next_step.block_depth - next_step.c_block.len());

                Option::Some(next_step)
            }
            _ => Option::None,
        },
        Amphipod::Desert => match game.is_d_block_valid() {
            true => {
                let mut next_step = game.clone();
                next_step.cd_buffer = Option::None;
                next_step.d_block.push(Amphipod::Desert);
                next_step.energy += 2000
                    + amphipod.multiplier() * (next_step.block_depth - next_step.d_block.len());
                Option::Some(next_step)
            }
            _ => Option::None,
        },
    })
}

fn right_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.right_buffer
        .iter()
        .find_map(|amphipod| match amphipod {
            Amphipod::Amber => match (
                game.is_a_block_valid(),
                game.ab_buffer,
                game.bc_buffer,
                game.cd_buffer,
            ) {
                (true, Option::None, Option::None, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.right_buffer = Option::None;
                    next_step.a_block.push(Amphipod::Amber);
                    next_step.energy += 8 + amphipod.multiplier()
                        * (next_step.block_depth - next_step.a_block.len());
                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Bronze => match (game.is_b_block_valid(), game.bc_buffer, game.cd_buffer) {
                (true, Option::None, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.right_buffer = Option::None;
                    next_step.b_block.push(Amphipod::Bronze);
                    next_step.energy += 60
                        + amphipod.multiplier() * (next_step.block_depth - next_step.b_block.len());

                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Copper => match (game.is_c_block_valid(), game.cd_buffer) {
                (true, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.right_buffer = Option::None;
                    next_step.c_block.push(Amphipod::Copper);
                    next_step.energy += 400
                        + amphipod.multiplier() * (next_step.block_depth - next_step.c_block.len());

                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Desert => match game.is_d_block_valid() {
                true => {
                    let mut next_step = game.clone();
                    next_step.right_buffer = Option::None;
                    next_step.d_block.push(Amphipod::Desert);
                    next_step.energy += 2000
                        + amphipod.multiplier() * (next_step.block_depth - next_step.d_block.len());
                    Option::Some(next_step)
                }
                _ => Option::None,
            },
        })
}

fn far_right_buffer_valid_moves(game: &AmphipodGame) -> Option<AmphipodGame> {
    game.far_right_buffer
        .iter()
        .find_map(|amphipod| match amphipod {
            Amphipod::Amber => match (
                game.is_a_block_valid(),
                game.ab_buffer,
                game.bc_buffer,
                game.cd_buffer,
                game.right_buffer,
            ) {
                (true, Option::None, Option::None, Option::None, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.far_right_buffer = Option::None;
                    next_step.a_block.push(Amphipod::Amber);
                    next_step.energy += 9 + amphipod.multiplier()
                        * (next_step.block_depth - next_step.a_block.len());
                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Bronze => match (
                game.is_b_block_valid(),
                game.bc_buffer,
                game.cd_buffer,
                game.right_buffer,
            ) {
                (true, Option::None, Option::None, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.far_right_buffer = Option::None;
                    next_step.b_block.push(Amphipod::Bronze);
                    next_step.energy += 70 + 10 * (next_step.block_depth - next_step.b_block.len());

                    Option::Some(next_step)
                }
                _ => Option::None,
            },
            Amphipod::Copper => {
                match (game.is_c_block_valid(), game.cd_buffer, game.right_buffer) {
                    (true, Option::None, Option::None) => {
                        let mut next_step = game.clone();
                        next_step.far_right_buffer = Option::None;
                        next_step.c_block.push(Amphipod::Copper);
                        next_step.energy += 500
                            + amphipod.multiplier()
                                * (next_step.block_depth - next_step.c_block.len());

                        Option::Some(next_step)
                    }
                    _ => Option::None,
                }
            }
            Amphipod::Desert => match (game.is_d_block_valid(), game.right_buffer) {
                (true, Option::None) => {
                    let mut next_step = game.clone();
                    next_step.far_right_buffer = Option::None;
                    next_step.d_block.push(Amphipod::Desert);
                    next_step.energy += 3000
                        + amphipod.multiplier() * (next_step.block_depth - next_step.d_block.len());
                    Option::Some(next_step)
                }
                _ => Option::None,
            },
        })
}

fn parse_data(input: &String) -> IResult<&str, (Vec<Amphipod>, Vec<Amphipod>)> {
    tuple((
        preceded(
            tag("#############\n#...........#\n###"),
            parse_amphipod_line,
        ),
        preceded(tag("###\n  #"), parse_amphipod_line),
    ))(input)
}

fn parse_amphipod_line(input: &str) -> IResult<&str, Vec<Amphipod>> {
    separated_list0(tag("#"), parse_amphipod)(input)
}

fn parse_amphipod(input: &str) -> IResult<&str, Amphipod> {
    alt((
        value(Amphipod::Amber, tag("A")),
        value(Amphipod::Bronze, tag("B")),
        value(Amphipod::Copper, tag("C")),
        value(Amphipod::Desert, tag("D")),
    ))(input)
}
