use crate::lib::{default_sub_command, parse_usize, CommandResult, Problem};
use clap::{App, Arg, ArgMatches};
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::newline,
    combinator::{map, map_parser},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub const GIANT_SQUID: Problem<GiantSquidArgs, BingoGame> = Problem::new(
    sub_command,
    "giant-squid",
    "day4_giant_squid",
    parse_arguments,
    parse_bingo_game,
    run,
);

#[derive(Debug)]
pub struct GiantSquidArgs {
    squid_win: bool,
}

#[derive(Debug, Clone)]
pub struct BingoGame {
    numbers_to_call: Vec<usize>,
    boards: Vec<BingoBoard>,
}

#[derive(Debug, Clone)]
struct BingoBoard {
    cells: Vec<Vec<BingoCell>>,
}

#[derive(Debug, Copy, Clone)]
struct BingoCell {
    number: usize,
    called: bool,
}

impl BingoCell {
    fn new(number: usize) -> BingoCell {
        BingoCell {
            number: number,
            called: false,
        }
    }
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &GIANT_SQUID,
        "Finds the best bingo board to play",
        "Path to the input file. The numbers to darw in order, followed by a set of boards.",
        "Finds the best board, sums the uncalled numbers then multiplies that by the last number that was called.",
        "Finds the worst borad, sums the uncalled numbers  hen multiplies that by the last number that was called.",
    )
    .arg(
        Arg::with_name("squid-win")
        .short("s")
        .help("If passed, try to let the squid win (find the worst board)."))
}

fn parse_arguments(arguments: &ArgMatches) -> GiantSquidArgs {
    match arguments.subcommand_name() {
        Some("part1") => GiantSquidArgs { squid_win: false },
        Some("part2") => GiantSquidArgs { squid_win: true },
        _ => GiantSquidArgs {
            squid_win: arguments.is_present("squid-win"),
        },
    }
}

fn run(arguments: GiantSquidArgs, bingo_game: BingoGame) -> CommandResult {
    process_bingo_winner(find_bingo_winner(
        bingo_game,
        select_winner(&arguments.squid_win),
    ))
    .into()
}

fn process_bingo_winner(winner: (BingoBoard, usize)) -> usize {
    let (board, last_number) = winner;

    board.cells.into_iter().fold(0, |acc, row| {
        acc + row
            .into_iter()
            .filter(|cell| !cell.called)
            .fold(0, |row_acc, cell| row_acc + cell.number)
    }) * last_number
}

fn find_bingo_winner(
    bingo_game: BingoGame,
    determine_winner: impl Fn(&Vec<BingoBoard>) -> bool,
) -> (BingoBoard, usize) {
    let mut boards = bingo_game.boards;
    let mut last_called_number = 0usize;

    for number in bingo_game.numbers_to_call.into_iter() {
        last_called_number = number;
        boards = boards
            .into_iter()
            .map(|board| BingoBoard {
                cells: board
                    .cells
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|cell| {
                                if cell.number == number {
                                    BingoCell {
                                        number: number,
                                        called: true,
                                    }
                                } else {
                                    cell
                                }
                            })
                            .collect()
                    })
                    .collect(),
            })
            .collect();

        if determine_winner(&boards) {
            break;
        }

        boards = boards
            .into_iter()
            .filter(|board| !is_board_winner(&board))
            .collect();
    }

    (
        boards.into_iter().find(is_board_winner).unwrap(),
        last_called_number,
    )
}

fn select_winner(squid_win: &bool) -> impl Fn(&Vec<BingoBoard>) -> bool {
    if *squid_win {
        is_last_winner
    } else {
        is_first_winner
    }
}

fn is_first_winner(boards: &Vec<BingoBoard>) -> bool {
    boards.into_iter().any(|board| is_board_winner(&board))
}

fn is_last_winner(boards: &Vec<BingoBoard>) -> bool {
    boards.into_iter().all(|board| is_board_winner(&board))
}

fn is_board_winner(bingo_board: &BingoBoard) -> bool {
    has_row_winner(bingo_board) || has_column_winner(bingo_board)
}

fn has_row_winner(bingo_board: &BingoBoard) -> bool {
    bingo_board
        .cells
        .iter()
        .any(|row| row.into_iter().all(|cell| cell.called))
}

fn has_column_winner(bingo_board: &BingoBoard) -> bool {
    for i in 0..bingo_board.cells.len() {
        let column_result = bingo_board
            .cells
            .iter()
            .map(|row| row.get(i).unwrap().called)
            .fold(true, |acc, called| acc && called);

        if column_result {
            return true;
        }
    }

    false
}

fn parse_bingo_game(input: &String) -> IResult<&str, BingoGame> {
    map(
        tuple((parse_numbers_to_call, parse_bingo_boards)),
        |(numbers_to_call, bingo_boards)| BingoGame {
            numbers_to_call: numbers_to_call,
            boards: bingo_boards,
        },
    )(input)
}

fn parse_bingo_boards(input: &str) -> IResult<&str, Vec<BingoBoard>> {
    separated_list0(tag("\n\n"), parse_bingo_board)(input)
}

fn parse_bingo_board(input: &str) -> IResult<&str, BingoBoard> {
    map(separated_list1(newline, parse_bingo_cell_row), |cells| {
        BingoBoard { cells: cells }
    })(input)
}

fn parse_bingo_cell_row(input: &str) -> IResult<&str, Vec<BingoCell>> {
    map(
        many1(preceded(take_while(|c| c == ' '), parse_usize)),
        |cells| cells.into_iter().map(BingoCell::new).collect(),
    )(input)
}

fn parse_numbers_to_call(input: &str) -> IResult<&str, Vec<usize>> {
    map_parser(
        terminated(take_until("\n\n"), tag("\n\n")),
        separated_list0(tag(","), parse_usize),
    )(input)
}
