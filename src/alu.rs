use adventofcode2021::{default_sub_command, parse_isize, CommandResult, Problem};
use clap::{values_t_or_exit, App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

pub const ALU: Problem<AluArgs, Vec<Instruction>> = Problem::new(
    sub_command,
    "alu",
    "day24_alu",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct AluArgs {
    inputs: Vec<isize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Variable(Variable),
    Literal(isize),
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &ALU,
        "Parses the input program then runs it using the supplied inputs. Prints all register values and returns the value in z.",
        "Path to the input file. Input should be newline delimited instructions.",
        "Runs the default program with the largest valid inputs.",
        "Runs the default program with the smallest valid inputs.",
    )
    .arg(
        Arg::with_name("input")
            .short("i")
            .help("Inputs to push into the alu program in the order they appear.")
            .multiple(true)
            .takes_value(true)
            .allow_hyphen_values(true)
            .number_of_values(1),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> AluArgs {
    match arguments.subcommand_name() {
        Some("part1") => AluArgs {
            inputs: vec![9, 9, 8, 9, 3, 9, 9, 9, 2, 9, 1, 9, 6, 7isize],
        },
        Some("part2") => AluArgs {
            inputs: vec![3, 4, 1, 7, 1, 9, 1, 1, 1, 8, 1, 2, 1, 1isize],
        },
        _ => AluArgs {
            inputs: values_t_or_exit!(arguments.values_of("input"), isize),
        },
    }
}

fn run(arguments: AluArgs, instructions: Vec<Instruction>) -> CommandResult {
    let (w, x, y, z) = interperate(&instructions, &arguments.inputs);

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);

    z.into()
}

fn interperate(
    instructions: &Vec<Instruction>,
    inputs: &Vec<isize>,
) -> (isize, isize, isize, isize) {
    // init
    let mut w = 0isize;
    let mut x = 0isize;
    let mut y = 0isize;
    let mut z = 0isize;
    let mut inputs = inputs.iter();

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Inp(variable) => {
                let next_input = *inputs.next().expect("Enough inputs for program");
                match variable {
                    Variable::W => w = next_input,
                    Variable::X => x = next_input,
                    Variable::Y => y = next_input,
                    Variable::Z => z = next_input,
                }
            }
            Instruction::Add(variable, value) => {
                let b = match value {
                    Value::Variable(variable) => match variable {
                        Variable::W => w,
                        Variable::X => x,
                        Variable::Y => y,
                        Variable::Z => z,
                    },
                    Value::Literal(input) => *input,
                };

                match variable {
                    Variable::W => w += b,
                    Variable::X => x += b,
                    Variable::Y => y += b,
                    Variable::Z => z += b,
                };
            }
            Instruction::Mul(variable, value) => {
                let b = match value {
                    Value::Variable(variable) => match variable {
                        Variable::W => w,
                        Variable::X => x,
                        Variable::Y => y,
                        Variable::Z => z,
                    },
                    Value::Literal(input) => *input,
                };

                match variable {
                    Variable::W => w *= b,
                    Variable::X => x *= b,
                    Variable::Y => y *= b,
                    Variable::Z => z *= b,
                };
            }
            Instruction::Div(variable, value) => {
                let b = match value {
                    Value::Variable(variable) => match variable {
                        Variable::W => w,
                        Variable::X => x,
                        Variable::Y => y,
                        Variable::Z => z,
                    },
                    Value::Literal(input) => *input,
                };

                match variable {
                    Variable::W => w /= b,
                    Variable::X => x /= b,
                    Variable::Y => y /= b,
                    Variable::Z => z /= b,
                };
            }
            Instruction::Mod(variable, value) => {
                let b = match value {
                    Value::Variable(variable) => match variable {
                        Variable::W => w,
                        Variable::X => x,
                        Variable::Y => y,
                        Variable::Z => z,
                    },
                    Value::Literal(input) => *input,
                };

                match variable {
                    Variable::W => w %= b,
                    Variable::X => x %= b,
                    Variable::Y => y %= b,
                    Variable::Z => z %= b,
                };
            }
            Instruction::Eql(variable, value) => {
                let b = match value {
                    Value::Variable(variable) => match variable {
                        Variable::W => w,
                        Variable::X => x,
                        Variable::Y => y,
                        Variable::Z => z,
                    },
                    Value::Literal(input) => *input,
                };

                match variable {
                    Variable::W => w = if w == b { 1isize } else { 0isize },
                    Variable::X => x = if x == b { 1isize } else { 0isize },
                    Variable::Y => y = if y == b { 1isize } else { 0isize },
                    Variable::Z => z = if z == b { 1isize } else { 0isize },
                };
            }
        });

    (w, x, y, z)
}

fn parse_data(input: &String) -> IResult<&str, Vec<Instruction>> {
    separated_list0(newline, parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_inp, parse_add, parse_mul, parse_div, parse_mod, parse_eql,
    ))(input)
}

fn parse_inp(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("inp "), parse_variable), |variable| {
        Instruction::Inp(variable)
    })(input)
}

fn parse_add(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("add "),
            separated_pair(parse_variable, tag(" "), parse_value),
        ),
        |(variable, value)| Instruction::Add(variable, value),
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("mul "),
            separated_pair(parse_variable, tag(" "), parse_value),
        ),
        |(variable, value)| Instruction::Mul(variable, value),
    )(input)
}

fn parse_div(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("div "),
            separated_pair(parse_variable, tag(" "), parse_value),
        ),
        |(variable, value)| Instruction::Div(variable, value),
    )(input)
}

fn parse_mod(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("mod "),
            separated_pair(parse_variable, tag(" "), parse_value),
        ),
        |(variable, value)| Instruction::Mod(variable, value),
    )(input)
}

fn parse_eql(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("eql "),
            separated_pair(parse_variable, tag(" "), parse_value),
        ),
        |(variable, value)| Instruction::Eql(variable, value),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(parse_variable, |variable| Value::Variable(variable)),
        map(parse_isize, |value| Value::Literal(value)),
    ))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Variable> {
    alt((
        value(Variable::W, tag("w")),
        value(Variable::X, tag("x")),
        value(Variable::Y, tag("y")),
        value(Variable::Z, tag("z")),
    ))(input)
}
