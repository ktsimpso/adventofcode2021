use adventofcode2021::{default_sub_command, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};
use std::{
    cmp::min,
    collections::{BTreeMap, HashMap, HashSet},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    buffers: BTreeMap<BufferLocation, Amphipod>,
    blocks: BTreeMap<Amphipod, Vec<Amphipod>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum BufferLocation {
    FarLeft,
    Left,
    AB,
    BC,
    CD,
    Right,
    FarRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
    Buffer(BufferLocation),
    Block(Amphipod),
}

impl Node {
    fn get_adjacent_nodes(&self) -> Vec<(Node, usize)> {
        match self {
            Node::Buffer(location) => match location {
                BufferLocation::FarLeft => vec![(Node::Buffer(BufferLocation::Left), 1)],
                BufferLocation::Left => vec![
                    (Node::Buffer(BufferLocation::FarLeft), 1),
                    (Node::Buffer(BufferLocation::AB), 2),
                    (Node::Block(Amphipod::Amber), 2),
                ],
                BufferLocation::AB => vec![
                    (Node::Buffer(BufferLocation::Left), 2),
                    (Node::Buffer(BufferLocation::BC), 2),
                    (Node::Block(Amphipod::Amber), 2),
                    (Node::Block(Amphipod::Bronze), 2),
                ],
                BufferLocation::BC => vec![
                    (Node::Buffer(BufferLocation::AB), 2),
                    (Node::Buffer(BufferLocation::CD), 2),
                    (Node::Block(Amphipod::Bronze), 2),
                    (Node::Block(Amphipod::Copper), 2),
                ],
                BufferLocation::CD => vec![
                    (Node::Buffer(BufferLocation::BC), 2),
                    (Node::Buffer(BufferLocation::Right), 2),
                    (Node::Block(Amphipod::Copper), 2),
                    (Node::Block(Amphipod::Desert), 2),
                ],
                BufferLocation::Right => vec![
                    (Node::Buffer(BufferLocation::FarRight), 1),
                    (Node::Buffer(BufferLocation::CD), 2),
                    (Node::Block(Amphipod::Desert), 2),
                ],
                BufferLocation::FarRight => vec![(Node::Buffer(BufferLocation::Right), 1)],
            },
            Node::Block(amphipod) => match amphipod {
                Amphipod::Amber => vec![
                    (Node::Buffer(BufferLocation::Left), 2),
                    (Node::Buffer(BufferLocation::AB), 2),
                ],
                Amphipod::Bronze => vec![
                    (Node::Buffer(BufferLocation::AB), 2),
                    (Node::Buffer(BufferLocation::BC), 2),
                ],
                Amphipod::Copper => vec![
                    (Node::Buffer(BufferLocation::BC), 2),
                    (Node::Buffer(BufferLocation::CD), 2),
                ],
                Amphipod::Desert => vec![
                    (Node::Buffer(BufferLocation::CD), 2),
                    (Node::Buffer(BufferLocation::Right), 2),
                ],
            },
        }
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
        buffers: BTreeMap::new(),
        blocks: BTreeMap::from([
            (Amphipod::Amber, a_block),
            (Amphipod::Bronze, b_block),
            (Amphipod::Copper, c_block),
            (Amphipod::Desert, d_block),
        ]),
    };

    let mut games = HashMap::from([(game, 0usize)]);
    let mut lowest_energy = usize::MAX;
    let mut losers = HashSet::new();

    while games.len() > 0 {
        let new_games: Vec<(AmphipodGame, usize)> = games
            .into_iter()
            .filter_map(|(game, energy)| {
                let moves = get_all_valid_moves(&game, energy);
                if moves.len() > 0 {
                    Option::Some(moves)
                } else {
                    losers.insert(game);
                    Option::None
                }
            })
            .flatten()
            .filter(|(game, energy)| {
                let winner = is_game_winner(&game);

                if winner {
                    lowest_energy = min(lowest_energy, *energy);
                }

                !winner
            })
            .collect();
        games = new_games
            .into_iter()
            .fold(HashMap::new(), |mut acc, (game, energy)| {
                if losers.contains(&game) {
                    return acc;
                }
                let result = min(*acc.get(&game).unwrap_or(&energy), energy);
                acc.insert(game, result);
                acc
            });
    }

    lowest_energy.into()
}

fn is_block_valid(amphipod: &Amphipod, block: &Vec<Amphipod>) -> bool {
    block.iter().all(|item| match item {
        x if x == amphipod => true,
        _ => false,
    })
}

fn is_game_winner(game: &AmphipodGame) -> bool {
    game.blocks
        .iter()
        .all(|(amphipod, block)| is_block_valid(amphipod, block))
        && game.buffers.is_empty()
}

fn get_all_valid_moves(game: &AmphipodGame, energy: usize) -> HashMap<AmphipodGame, usize> {
    let mut valid_moves: HashMap<AmphipodGame, usize> = game
        .blocks
        .keys()
        .map(|block| Node::Block(*block))
        .map(|node| get_valid_moves(&game, energy, node))
        .fold(HashMap::new(), |mut acc, moves| {
            moves.into_iter().for_each(|(game, energy)| {
                let result = min(*acc.get(&game).unwrap_or(&energy), energy);
                acc.insert(game, result);
            });
            acc
        });
    valid_moves = game
        .buffers
        .keys()
        .map(|location| Node::Buffer(*location))
        .map(|node| get_valid_moves(&game, energy, node))
        .fold(valid_moves, |mut acc, moves| {
            moves.into_iter().for_each(|(game, energy)| {
                let result = min(*acc.get(&game).unwrap_or(&energy), energy);
                acc.insert(game, result);
            });
            acc
        });

    valid_moves
}

fn get_valid_moves(game: &AmphipodGame, energy: usize, node: Node) -> HashMap<AmphipodGame, usize> {
    let (move_amphipod, base_cost, new_base_game, can_go_to_buffer) = match &node {
        Node::Block(amphipod) => {
            let block = game.blocks.get(&amphipod).expect("Block exists");
            if is_block_valid(&amphipod, block) {
                return HashMap::new();
            }

            let mut new_base_game = game.clone();
            let new_block = new_base_game
                .blocks
                .get_mut(amphipod)
                .expect("Block exists");
            new_block.pop();

            (
                block
                    .last()
                    .expect("Empty a block would trigger valid check"),
                game.block_depth - block.len(),
                new_base_game,
                true,
            )
        }
        Node::Buffer(location) => {
            let buffer = game.buffers.get(&location);
            if buffer.is_none() {
                return HashMap::new();
            }

            let mut new_base_game = game.clone();
            new_base_game.buffers.remove(location);

            (buffer.expect("None checked"), 0usize, new_base_game, false)
        }
    };

    let mut queue: Vec<(Node, usize)> = node
        .get_adjacent_nodes()
        .into_iter()
        .map(|(node, cost)| (node, cost + base_cost))
        .collect();
    let mut seen = HashSet::from([node]);
    queue
        .iter()
        .map(|(node, _)| node.to_owned())
        .for_each(|node| {
            seen.insert(node);
        });
    let mut games = HashMap::new();

    while let Some((node, cost)) = queue.pop() {
        match node {
            Node::Block(amphipod) => {
                if move_amphipod == &amphipod {
                    let block = game.blocks.get(&amphipod).expect("Block exists");
                    if is_block_valid(&amphipod, block) {
                        let mut new_game = new_base_game.clone();
                        let new_block = new_game.blocks.get_mut(&amphipod).expect("Block exists");
                        new_block.push(*move_amphipod);

                        let final_cost = game.block_depth - new_block.len() + cost;
                        let final_energy = energy + final_cost * move_amphipod.multiplier();
                        games.insert(new_game, final_energy);
                    }
                }
            }
            Node::Buffer(location) => {
                if !game.buffers.contains_key(&location) {
                    if can_go_to_buffer {
                        let mut new_game = new_base_game.clone();
                        new_game.buffers.insert(location, *move_amphipod);
                        let final_energy = energy + cost * move_amphipod.multiplier();
                        games.insert(new_game, final_energy);
                    }

                    let new_nodes: Vec<(Node, usize)> = node
                        .get_adjacent_nodes()
                        .into_iter()
                        .filter(|(node, _)| !seen.contains(node))
                        .map(|(node, node_cost)| (node, cost + node_cost))
                        .collect();

                    new_nodes.iter().for_each(|(node, cost)| {
                        queue.push((*node, *cost));
                        seen.insert(*node);
                    });
                }
            }
        }

        seen.insert(node);
    }

    games
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
