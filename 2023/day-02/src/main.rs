use std::ops::Deref;

use anyhow::{bail, Context, Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../input1.txt");

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Vec<Game> = input
        .lines()
        .map(|s| parse_game(s))
        .collect::<Result<_>>()?;

    let sum: u32 = games
        .into_iter()
        .filter(|g| g.iter().all(|b| b.is_contained(&bag)))
        .map(|g| g.get_id())
        .sum();

    println!("The sum of the ids of the valid games is {sum}");

    Ok(())
}

/// A game of the Cubes Game
/// Each game consists of an id and successive cubes pulled out of the bag
/// Each pull is represented as a bag itself
struct Game(u32, Vec<Bag>);

impl Deref for Game {
    type Target = Vec<Bag>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl Game {
    fn get_id(&self) -> u32 {
        return self.0;
    }
}

/// A bag from the Cubes Game
/// Each bag has an ammount of red green and blue cubes
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn is_contained(&self, other: &Bag) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

/// Parses a Cubes Game
/// Example Input:
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// The number before ':' is the id, then we have a ';' separated list of bags
fn parse_game(game: &str) -> Result<Game> {
    let (id, bags) = game
        .split_once(":")
        .context("The ':' must separate the id from the bags")?;

    let id: u32 = id
        .trim()
        .split_once(" ")
        .context("The id must be 'Game n'")?
        .1
        .parse()?;

    let bags: Vec<Bag> = bags
        .split(";")
        .map(|s| parse_bag(s))
        .collect::<Result<_>>()?;

    Ok(Game(id, bags))
}

/// Parses a bag
/// Example Input:
/// 3 blue, 4 red, 8 green
#[inline]
fn parse_bag(cubes: &str) -> Result<Bag> {
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };

    let colored_cubes = cubes.split(",").map(|s| parse_colored_cubes(s));

    for cubes in colored_cubes {
        let (color, n) = cubes?;

        match color {
            Color::Red => bag.red = n,
            Color::Green => bag.green = n,
            Color::Blue => bag.blue = n,
        }
    }

    Ok(bag)
}

#[inline]
/// Parses a many cubes of the same color
/// Example Input
/// 3 blue
///
/// Returns: a tuple of (color, quantity)
fn parse_colored_cubes(cubes: &str) -> Result<(Color, u32)> {
    let (n, color) = cubes
        .trim()
        .split_once(" ")
        .context("Cubes string should only have one space")?;

    let color = match color {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => bail!("Invalid color string"),
    };

    let n: u32 = n.parse().context("Should be a number")?;

    Ok((color, n))
}
