use crate::lib::{default_sub_command, parse_isize, parse_usize, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use num_integer::Roots;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const BEACON_SCANNER: Problem<BeaconScannerArgs, Vec<Scanner>> = Problem::new(
    sub_command,
    "beacon-scanner",
    "day19_beacon_scanner",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct BeaconScannerArgs {
    signal: Signal,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Signal {
    BeaconCount,
    MaxScannerDistance,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone)]
pub struct Scanner {
    beacons: Vec<Point>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &BEACON_SCANNER,
        "Coorilates all beacon and scanner signals and computes stats.",
        "Path to the input file. Input should be scanner results.",
        "Finds the total number of beacons for the default input.",
        "Finds the largest distance between two scanners for the default input.",
    ).arg(
        Arg::with_name("signal")
            .short("s")
            .help(
                "The signal to determine. The questions available are as follows:\n\n\
            beacon-count: Returns the total number of beacons.\n\n\
            max-scanner-distance: Returns the maximum hamiltonian distance between all scanners.\n\n",
            )
            .takes_value(true)
            .possible_values(&Signal::VARIANTS)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> BeaconScannerArgs {
    match arguments.subcommand_name() {
        Some("part1") => BeaconScannerArgs {
            signal: Signal::BeaconCount,
        },
        Some("part2") => BeaconScannerArgs {
            signal: Signal::MaxScannerDistance,
        },
        _ => BeaconScannerArgs {
            signal: value_t_or_exit!(arguments.value_of("signal"), Signal),
        },
    }
}

fn run(arguments: BeaconScannerArgs, mut scanners: Vec<Scanner>) -> CommandResult {
    let reference = scanners.remove(0);
    let mut scanner_points = vec![Point { x: 0, y: 0, z: 0 }];
    let mut beacons: HashSet<Point> = reference
        .beacons
        .iter()
        .map(|point| point.to_owned())
        .collect();

    while scanners.len() > 0 {
        match scanners.iter().enumerate().find_map(|(index, scanner)| {
            does_scanner_overlap(&beacons, scanner).map(|result| (result, index))
        }) {
            Option::Some(((reference_point, scanner_point, scanner), index)) => {
                get_beacon_rotations()
                    .iter()
                    .map(|rotation| {
                        let results: HashSet<Point> =
                            scanner.beacons.iter().map(|s| rotation(s)).collect();
                        (rotation(&scanner_point), results)
                    })
                    .map(|(scanner_ref, points)| {
                        let x_diff = scanner_ref.x - reference_point.x;
                        let y_diff = scanner_ref.y - reference_point.y;
                        let z_diff = scanner_ref.z - reference_point.z;

                        let rotated_points: HashSet<Point> = points
                            .iter()
                            .map(|point| Point {
                                x: point.x - x_diff,
                                y: point.y - y_diff,
                                z: point.z - z_diff,
                            })
                            .collect();

                        (
                            rotated_points,
                            Point {
                                x: -x_diff,
                                y: -y_diff,
                                z: -z_diff,
                            },
                        )
                    })
                    .find(|(rotations, scanner_position)| {
                        let mut new_beacons = beacons.clone();
                        new_beacons.extend(rotations);
                        if new_beacons.len() < (beacons.len() + rotations.len() - 11) {
                            beacons = new_beacons;
                            scanners.remove(index);
                            scanner_points.push(scanner_position.to_owned());
                            true
                        } else {
                            false
                        }
                    })
            }
            Option::None => Option::None,
        };
    }

    match arguments.signal {
        Signal::BeaconCount => beacons.len().into(),
        Signal::MaxScannerDistance => {
            let mut maximum = 0isize;
            for i in 0..(scanner_points.len() - 1) {
                for j in (i + 1)..scanner_points.len() {
                    let left = scanner_points.get(i).expect("Bounds checked");
                    let right = scanner_points.get(j).expect("Bounds checked");
                    let result = (left.x - right.x).abs()
                        + (left.y - right.y).abs()
                        + (left.z - right.z).abs();
                    maximum = max(result, maximum);
                }
            }
            maximum.into()
        }
    }
}

fn does_scanner_overlap(
    beacons: &HashSet<Point>,
    scanner: &Scanner,
) -> Option<(Point, Point, Scanner)> {
    beacons.iter().find_map(|fixed_point| {
        let reference = beacons
            .iter()
            .map(|point| distance(fixed_point, point))
            .fold(HashMap::new(), |mut acc, distance| {
                *acc.entry(distance).or_insert(0usize) += 1;
                acc
            });

        scanner
            .beacons
            .iter()
            .map(|scanner_fixed_point| {
                (
                    scanner
                        .beacons
                        .iter()
                        .map(|point| distance(scanner_fixed_point, point))
                        .fold(HashMap::new(), |mut acc, distance| {
                            *acc.entry(distance).or_insert(0usize) += 1;
                            acc
                        }),
                    scanner_fixed_point,
                )
            })
            .find(|(distances, _)| {
                reference
                    .iter()
                    .map(|(key, count)| min(count, distances.get(key).unwrap_or(&0usize)))
                    .fold(0usize, |acc, next| acc + *next)
                    >= 12
            })
            .map(|(_, scanner_fixed_point)| {
                (
                    fixed_point.to_owned(),
                    scanner_fixed_point.to_owned(),
                    scanner.to_owned(),
                )
            })
    })
}

fn get_beacon_rotations() -> Vec<Box<dyn Fn(&Point) -> Point>> {
    vec![
        // Face x
        Box::new(face_x_up_y),
        Box::new(face_x_up_negative_y),
        Box::new(face_x_up_z),
        Box::new(face_x_up_negative_z),
        // Face -x
        Box::new(face_negative_x_up_y),
        Box::new(face_negative_x_up_negative_y),
        Box::new(face_negative_x_up_z),
        Box::new(face_negative_x_up_negative_z),
        // Face y
        Box::new(face_y_up_x),
        Box::new(face_y_up_negative_x),
        Box::new(face_y_up_z),
        Box::new(face_y_up_negative_z),
        // Face -y
        Box::new(face_negative_y_up_x),
        Box::new(face_negative_y_up_negative_x),
        Box::new(face_negative_y_up_z),
        Box::new(face_negative_y_up_negative_z),
        // Face z
        Box::new(face_z_up_x),
        Box::new(face_z_up_negative_x),
        Box::new(face_z_up_y),
        Box::new(face_z_up_negative_y),
        // Face -z
        Box::new(face_negative_z_up_x),
        Box::new(face_negative_z_up_negative_x),
        Box::new(face_negative_z_up_y),
        Box::new(face_negative_z_up_negative_y),
    ]
}

fn face_x_up_y(point: &Point) -> Point {
    point.to_owned()
}

fn face_x_up_negative_y(point: &Point) -> Point {
    Point {
        x: point.x,
        y: -point.y,
        z: -point.z,
    }
}

fn face_x_up_z(point: &Point) -> Point {
    Point {
        x: point.x,
        y: point.z,
        z: -point.y,
    }
}

fn face_x_up_negative_z(point: &Point) -> Point {
    Point {
        x: point.x,
        y: -point.z,
        z: point.y,
    }
}

fn face_negative_x_up_y(point: &Point) -> Point {
    Point {
        x: -point.x,
        y: -point.y,
        z: point.z,
    }
}

fn face_negative_x_up_negative_y(point: &Point) -> Point {
    Point {
        x: -point.x,
        y: point.y,
        z: -point.z,
    }
}

fn face_negative_x_up_z(point: &Point) -> Point {
    Point {
        x: -point.x,
        y: point.z,
        z: point.y,
    }
}

fn face_negative_x_up_negative_z(point: &Point) -> Point {
    Point {
        x: -point.x,
        y: -point.z,
        z: -point.y,
    }
}

fn face_y_up_x(point: &Point) -> Point {
    Point {
        x: point.y,
        y: point.x,
        z: -point.z,
    }
}

fn face_y_up_negative_x(point: &Point) -> Point {
    Point {
        x: point.y,
        y: -point.x,
        z: point.z,
    }
}

fn face_y_up_z(point: &Point) -> Point {
    Point {
        x: point.y,
        y: point.z,
        z: point.x,
    }
}

fn face_y_up_negative_z(point: &Point) -> Point {
    Point {
        x: point.y,
        y: -point.z,
        z: -point.x,
    }
}

fn face_negative_y_up_x(point: &Point) -> Point {
    Point {
        x: -point.y,
        y: point.x,
        z: point.z,
    }
}

fn face_negative_y_up_negative_x(point: &Point) -> Point {
    Point {
        x: -point.y,
        y: -point.x,
        z: -point.z,
    }
}

fn face_negative_y_up_z(point: &Point) -> Point {
    Point {
        x: -point.y,
        y: point.z,
        z: -point.x,
    }
}

fn face_negative_y_up_negative_z(point: &Point) -> Point {
    Point {
        x: -point.y,
        y: -point.z,
        z: point.x,
    }
}

fn face_z_up_x(point: &Point) -> Point {
    Point {
        x: point.z,
        y: point.x,
        z: point.y,
    }
}

fn face_z_up_negative_x(point: &Point) -> Point {
    Point {
        x: point.z,
        y: -point.x,
        z: -point.y,
    }
}

fn face_z_up_y(point: &Point) -> Point {
    Point {
        x: point.z,
        y: point.y,
        z: -point.x,
    }
}

fn face_z_up_negative_y(point: &Point) -> Point {
    Point {
        x: point.z,
        y: -point.y,
        z: point.x,
    }
}

fn face_negative_z_up_x(point: &Point) -> Point {
    Point {
        x: -point.z,
        y: point.x,
        z: -point.y,
    }
}

fn face_negative_z_up_negative_x(point: &Point) -> Point {
    Point {
        x: -point.z,
        y: -point.x,
        z: point.y,
    }
}

fn face_negative_z_up_y(point: &Point) -> Point {
    Point {
        x: -point.z,
        y: point.y,
        z: point.x,
    }
}

fn face_negative_z_up_negative_y(point: &Point) -> Point {
    Point {
        x: -point.z,
        y: -point.y,
        z: -point.x,
    }
}

fn distance(point1: &Point, point2: &Point) -> isize {
    let dx = point2.x - point1.x;
    let dy = point2.y - point1.y;
    let dz = point2.z - point1.z;
    ((dx * dx) + (dy * dy) + (dz * dz)).sqrt()
}

fn parse_data(input: &String) -> IResult<&str, Vec<Scanner>> {
    separated_list0(tag("\n\n"), parse_scanner)(input)
}

fn parse_scanner(input: &str) -> IResult<&str, Scanner> {
    map(
        tuple((
            terminated(parse_scanner_number, newline),
            separated_list0(newline, parse_point),
        )),
        |(_, points)| Scanner { beacons: points },
    )(input)
}

fn parse_scanner_number(input: &str) -> IResult<&str, usize> {
    delimited(tag("--- scanner "), parse_usize, tag(" ---"))(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        tuple((
            terminated(parse_isize, tag(",")),
            terminated(parse_isize, tag(",")),
            parse_isize,
        )),
        |(x, y, z)| Point { x: x, y: y, z: z },
    )(input)
}
