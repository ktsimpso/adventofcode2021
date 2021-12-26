use adventofcode2021::{default_sub_command, CommandResult, Problem};
use clap::{value_t_or_exit, App, Arg, ArgMatches};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::{many1, separated_list0},
    sequence::separated_pair,
    IResult,
};

pub const TRENCH_MAP: Problem<TrenchMapArgs, TrenchMap> = Problem::new(
    sub_command,
    "trench-map",
    "day20_trench_map",
    parse_arguments,
    parse_data,
    run,
);

#[derive(Debug)]
pub struct TrenchMapArgs {
    n: usize,
}

#[derive(Debug)]
pub struct TrenchMap {
    image_enhancement_algorithm: Vec<Pixel>,
    image: Vec<Vec<Pixel>>,
}

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &TRENCH_MAP,
        "Ecnhances an image then counts the number of Light pixels in the images.",
        "Path to the input file. Input should be the image enhancement algorithm followed by the image data.",
        "Enchances the image in the default input twice.",
        "Enchances the image in the default input 50 times.",
    ).arg(
        Arg::with_name("number")
            .short("n")
            .help("Number of times to enchance the image.")
            .takes_value(true)
            .required(true),
    )
}

fn parse_arguments(arguments: &ArgMatches) -> TrenchMapArgs {
    match arguments.subcommand_name() {
        Some("part1") => TrenchMapArgs { n: 2usize },
        Some("part2") => TrenchMapArgs { n: 50usize },
        _ => TrenchMapArgs {
            n: value_t_or_exit!(arguments.value_of("number"), usize),
        },
    }
}

fn run(arguments: TrenchMapArgs, trench_map: TrenchMap) -> CommandResult {
    let mut new_image = trench_map.image.clone();
    let mut expand_pixels = Pixel::Dark;

    for _ in 0..arguments.n {
        new_image = expand_image(&new_image, &expand_pixels);
        new_image = new_image
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        map_pixel_to_real_pixel(
                            x,
                            y,
                            &new_image,
                            &trench_map.image_enhancement_algorithm,
                            &expand_pixels,
                        )
                    })
                    .collect()
            })
            .collect();
        expand_pixels = map_pixel_set_to_new_pixel(
            &vec![expand_pixels; 9],
            &trench_map.image_enhancement_algorithm,
        );
    }

    (new_image
        .iter()
        .map(|row| {
            row.iter()
                .filter(|pixel| match pixel {
                    Pixel::Dark => false,
                    Pixel::Light => true,
                })
                .count()
        })
        .fold(0usize, |acc, light_count| acc + light_count))
    .into()
}

fn expand_image(image: &Vec<Vec<Pixel>>, expand_pixels: &Pixel) -> Vec<Vec<Pixel>> {
    let desired_x = image.first().expect("At least one row").len() + 2;
    let top_bottom_rows = vec![*expand_pixels; desired_x];
    let mut new_image = vec![top_bottom_rows.clone()];

    new_image.extend(image.iter().map(|row| {
        let mut new_row = vec![*expand_pixels];
        new_row.extend(row.iter());
        new_row.push(*expand_pixels);
        new_row
    }));
    new_image.push(top_bottom_rows);
    new_image
}

fn map_pixel_to_real_pixel(
    x: usize,
    y: usize,
    image: &Vec<Vec<Pixel>>,
    image_enhancement_algorithm: &Vec<Pixel>,
    default: &Pixel,
) -> Pixel {
    map_pixel_set_to_new_pixel(
        &get_adjacent_pixels(&image, x, y, default),
        &image_enhancement_algorithm,
    )
}

fn map_pixel_set_to_new_pixel(
    pixel_set: &Vec<Pixel>,
    image_enhancement_algorithm: &Vec<Pixel>,
) -> Pixel {
    let bits = pixel_set
        .iter()
        .map(|pixel| match pixel {
            Pixel::Light => "1",
            Pixel::Dark => "0",
        })
        .collect::<Vec<&str>>()
        .join("");

    image_enhancement_algorithm
        .get(usize::from_str_radix(&bits, 2).unwrap())
        .unwrap()
        .to_owned()
}

fn get_adjacent_pixels(pixel: &Vec<Vec<Pixel>>, x: usize, y: usize, default: &Pixel) -> Vec<Pixel> {
    let mut pixels = Vec::new();

    pixels.push(if y > 0usize && x > 0usize {
        *pixel
            .get(y - 1)
            .and_then(|result| result.get(x - 1))
            .unwrap_or(default)
    } else {
        *default
    });

    pixels.push(if y > 0usize {
        *pixel
            .get(y - 1)
            .and_then(|result| result.get(x))
            .unwrap_or(default)
    } else {
        *default
    });

    pixels.push(if y > 0usize {
        *pixel
            .get(y - 1)
            .and_then(|result| result.get(x + 1))
            .unwrap_or(default)
    } else {
        *default
    });

    pixels.push(if x > 0usize {
        *pixel
            .get(y)
            .and_then(|result| result.get(x - 1))
            .unwrap_or(default)
    } else {
        *default
    });

    pixels.push(
        *pixel
            .get(y)
            .and_then(|result| result.get(x))
            .unwrap_or(default),
    );

    pixels.push(
        *pixel
            .get(y)
            .and_then(|result| result.get(x + 1))
            .unwrap_or(default),
    );

    pixels.push(if x > 0usize {
        *pixel
            .get(y + 1)
            .and_then(|result| result.get(x - 1))
            .unwrap_or(default)
    } else {
        *default
    });

    pixels.push(
        *pixel
            .get(y + 1)
            .and_then(|result| result.get(x))
            .unwrap_or(default),
    );

    pixels.push(
        *pixel
            .get(y + 1)
            .and_then(|result| result.get(x + 1))
            .unwrap_or(default),
    );

    pixels
}

fn parse_data(input: &String) -> IResult<&str, TrenchMap> {
    map(
        separated_pair(
            parse_pixel_line,
            tag("\n\n"),
            separated_list0(newline, parse_pixel_line),
        ),
        |(image_enhancement_algorithm, image)| TrenchMap {
            image_enhancement_algorithm: image_enhancement_algorithm,
            image: image,
        },
    )(input)
}

fn parse_pixel_line(input: &str) -> IResult<&str, Vec<Pixel>> {
    many1(parse_pixel)(input)
}

fn parse_pixel(input: &str) -> IResult<&str, Pixel> {
    alt((value(Pixel::Dark, tag(".")), value(Pixel::Light, tag("#"))))(input)
}
