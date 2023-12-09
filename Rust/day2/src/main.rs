use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use nom::bytes::complete::take_while;
use nom::character::is_alphabetic;

use std::error::Error;

#[derive(Debug, PartialEq)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubes {
    // fn from(s: &str) -> Self {
    //
    // }
}

struct Game {
    id: usize,
    elements: Vec<Cubes>,
}

impl Game {
    fn new_from(lines: &[&str]) -> Self {
        for line in lines {
            let games: Vec<_> = line.split(&[';', ':']).collect();
            // println!("{games:?}");
            for (game_id, game) in games.iter().enumerate() {
                println!("{game:?}");
            }
        }
        Game {
            id: 0,
            elements: vec![],
        }
    }
}

// "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"

// fn parse_cubes(input: &str) -> IResult<&str, &str> {
//     let (input, _) = tag(":")(input)?;
//     Ok(tag(";")(input)?)
//     // let (input, (red, green, blue)) = (())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input_small.txt")?;

    println!("{input:?}");

    Ok(())
}
