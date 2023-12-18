use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use std::collections::BTreeMap;
use std::error::Error;
use std::ops::Not;

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

fn cube_sorting(input: &Cube) -> u32 {
    input.amount
}

impl<'a> Game<'a> {
    fn is_valid(&self, map: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .any(|round| {
                round.iter().any(|shown_cube| {
                    shown_cube.amount > *map.get(shown_cube.color).expect("a valid cube")
                })
            })
            .not()
            .then_some(
                self.id
                    .parse::<u32>()
                    .expect("game id should a parsable u32"),
            )
    }

    fn is_valid2(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| {
                            *v = (*v).max(cube.amount);
                        })
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = parse_games(input).expect("this should parse !!!");

    println!("{games:?}");
    Ok(games
        .1
        .iter()
        .filter_map(|game| game.is_valid(&map))
        .sum::<u32>()
        .try_into()
        .unwrap())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let games = parse_games(input).expect("this should parse the games!!!");
    Ok(games
        .1
        .iter()
        .map(|game| game.is_valid2())
        .sum::<u32>()
        .try_into()
        .unwrap())
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}
// 3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue;
// 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { rounds, id }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");

    let result1 = part1(&input)?;
    let result2 = part2(&input)?;

    println!("result part1: {result1:?}");
    println!("result part2: {result2:?}");

    Ok(())
}
