use crate::lib::{
    complete_parsing, default_sub_command, file_to_string, parse_usize, Command, CommandResult,
};
use anyhow::Error;
use clap::{App, ArgMatches};
use nom::{
    bytes::complete::{tag, take_until, take_while},
    combinator::{map, map_parser},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub const GIANT_SQUID: Command = Command::new(sub_command, "giant-squid", "day4_giant_squid", run);

#[derive(Debug, Clone)]
struct BingoGame {
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
        "Finds the best board, sums the uncalled numbers then multiplies that by the number that was called.",
        "I will find out",
    )
}

fn run(arguments: &ArgMatches, file: &String) -> Result<CommandResult, Error> {
    file_to_string(file)
        .and_then(|f| complete_parsing(parse_bingo_game)(&f))
        .map(find_bingo_winner)
        .map(process_bingo_winner)
        .map(CommandResult::from)
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

fn find_bingo_winner(bingo_game: BingoGame) -> (BingoBoard, usize) {
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

        if boards
            .clone()
            .into_iter()
            .any(|board| is_board_winner(&board))
        {
            break;
        }
    }

    (
        boards.into_iter().find(is_board_winner).unwrap(),
        last_called_number,
    )
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
            .clone()
            .into_iter()
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
    map(separated_list1(tag("\n"), parse_bingo_cell_row), |cells| {
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
