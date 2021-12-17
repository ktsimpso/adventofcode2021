use crate::lib::{default_sub_command, CommandResult, Problem};
use clap::{App, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{flat_map, map, map_parser, map_res, value},
    multi::{count, many0, many_till},
    sequence::{preceded, tuple},
    IResult,
};

pub const PACKET_DECODER: Problem<PacketDecoderArgs, Packet> = Problem::new(
    sub_command,
    "packet-decoder",
    "day16_packet_decoder",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct PacketDecoderArgs {}

#[derive(Debug)]
pub struct Packet {
    version: usize,
    type_id: usize,
    packet_contents: PacketContents,
}

#[derive(Debug)]
enum PacketContents {
    Literal { value: usize },
    Operator { sub_packets: Vec<Packet> },
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &PACKET_DECODER,
        "Parses a packet then performs some operation on the result",
        "Hex encode string of the packet.",
        "Parses the packet, then sums all the versions inside.",
        "I will find out.",
    )
}

fn parse_arguments(arguments: &ArgMatches) -> PacketDecoderArgs {
    match arguments.subcommand_name() {
        Some("part1") => PacketDecoderArgs {},
        Some("part2") => PacketDecoderArgs {},
        _ => PacketDecoderArgs {},
    }
}

fn run(arguments: PacketDecoderArgs, packet: Packet) -> CommandResult {
    sum_packet_versions(&packet).into()
}

fn sum_packet_versions(packet: &Packet) -> usize {
    match &packet.packet_contents {
        PacketContents::Literal { value: _ } => packet.version,
        PacketContents::Operator { sub_packets } => {
            sub_packets
                .iter()
                .map(sum_packet_versions)
                .fold(0usize, |acc, result| acc + result)
                + packet.version
        }
    }
}

fn parse_data(input: &String) -> IResult<&str, Packet> {
    map_res(many0(parse_hex), |results| {
        let result = results.concat();
        let parse_result = parse_packet(Box::leak(result.into_boxed_str()));
        parse_result.map(|(_, packet)| packet)
    })(input)
}

fn parse_hex(input: &str) -> IResult<&str, &str> {
    alt((
        value("0000", tag("0")),
        value("0001", tag("1")),
        value("0010", tag("2")),
        value("0011", tag("3")),
        value("0100", tag("4")),
        value("0101", tag("5")),
        value("0110", tag("6")),
        value("0111", tag("7")),
        value("1000", tag("8")),
        value("1001", tag("9")),
        value("1010", tag("A")),
        value("1011", tag("B")),
        value("1100", tag("C")),
        value("1101", tag("D")),
        value("1110", tag("E")),
        value("1111", tag("F")),
    ))(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(
        tuple((
            parse_packet_version,
            flat_map(parse_type_id, parse_packet_info),
        )),
        |(version, (type_id, packet_contents))| Packet {
            version: version,
            type_id: type_id,
            packet_contents: packet_contents,
        },
    )(input)
}

fn parse_packet_version(input: &str) -> IResult<&str, usize> {
    map_res(take(3usize), |bits| usize::from_str_radix(bits, 2))(input)
}

fn parse_type_id(input: &str) -> IResult<&str, usize> {
    map_res(take(3usize), |bits| usize::from_str_radix(bits, 2))(input)
}

fn parse_packet_info(type_id: usize) -> impl Fn(&str) -> IResult<&str, (usize, PacketContents)> {
    move |input| {
        if type_id == 4 {
            map(parse_literal, |contents| (type_id, contents))(input)
        } else {
            map(parse_sub_packets, |contents| (type_id, contents))(input)
        }
    }
}

fn parse_literal(input: &str) -> IResult<&str, PacketContents> {
    map(
        map_res(
            many_till(
                preceded(tag("1"), take(4usize)),
                preceded(tag("0"), take(4usize)),
            ),
            |(list, last)| {
                let mut result = list.join("");
                result.push_str(last);
                usize::from_str_radix(&result, 2)
            },
        ),
        |value| PacketContents::Literal { value: value },
    )(input)
}

fn parse_sub_packets(input: &str) -> IResult<&str, PacketContents> {
    map(
        alt((
            map_parser(
                flat_map(
                    map_res(preceded(tag("0"), take(15usize)), |bits| {
                        usize::from_str_radix(bits, 2)
                    }),
                    take,
                ),
                many0(parse_packet),
            ),
            flat_map(
                map_res(preceded(tag("1"), take(11usize)), |bits| {
                    usize::from_str_radix(bits, 2)
                }),
                parse_n_packets,
            ),
        )),
        |sub_packets| PacketContents::Operator {
            sub_packets: sub_packets,
        },
    )(input)
}

fn parse_n_packets(n: usize) -> impl Fn(&str) -> IResult<&str, Vec<Packet>> {
    move |input| count(parse_packet, n)(input)
}
