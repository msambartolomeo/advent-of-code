use std::ops::Deref;

use anyhow::{bail, Context, Result};

/// A game of the Cube Conundrum
/// Each game consists of an id and successive cubes pulled out of the bag
/// Each pull is represented as a bag itself
#[derive(Debug, PartialEq, Eq)]
pub struct CubeConundrum(u32, Vec<Bag>);

impl Deref for CubeConundrum {
    type Target = Vec<Bag>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl CubeConundrum {
    #[must_use]
    pub fn get_id(&self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn minimum_bag(&self) -> Bag {
        let mut min = Bag::default();

        for bag in self.iter() {
            min.max_mut(bag);
        }

        min
    }
}

/// A bag from the Cube Conundrum game
/// Each bag has an ammount of red green and blue cubes
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    #[must_use]
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    #[must_use]
    pub fn is_contained(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    pub fn max_mut(&mut self, other: &Self) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }

    #[must_use]
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

/// Parses a List of Cube Conundrum games
///
/// Each game is an id and then a list of plays taking boxes out of a bag
///
/// # Input example
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n
///
/// # Errors
///
/// The funcion can fail if the string does not follow the example
pub fn parse_games(games: &str) -> Result<Vec<CubeConundrum>> {
    games.lines().map(parse_game).collect()
}

/// Parses a Cube Conundrum game
/// Example Input:
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// The number before ':' is the id, then we have a ';' separated list of bags
fn parse_game(game: &str) -> Result<CubeConundrum> {
    let (id, bags) = game
        .split_once(':')
        .context("The ':' must separate the id from the bags")?;

    let id: u32 = id
        .trim()
        .split_once(' ')
        .context("The id must be 'Game n'")?
        .1
        .parse()?;

    let bags: Vec<Bag> = bags.split(';').map(parse_bag).collect::<Result<_>>()?;

    Ok(CubeConundrum(id, bags))
}

/// Parses a bag
/// Example Input:
/// 3 blue, 4 red, 8 green
#[inline]
fn parse_bag(cubes: &str) -> Result<Bag> {
    let mut bag = Bag::default();

    let colored_cubes = cubes.split(',').map(parse_colored_cubes);

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
        .split_once(' ')
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colored_cubes() {
        let input = "3 blue";

        let result = parse_colored_cubes(input).expect("Invalid input");

        let (color, n) = result;

        assert_eq!(Color::Blue, color);
        assert_eq!(3, n);
    }

    #[test]
    fn test_bag() {
        let input = "3 blue, 4 red, 8 green";

        let result = parse_bag(input).expect("Invalid input");

        let expected = Bag::new(4, 8, 3);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_min_bag() {
        let mut b1 = Bag::new(6, 3, 1);
        let b2 = Bag::new(1, 2, 2);

        b1.max_mut(&b2);

        let expected = Bag::new(6, 3, 2);

        assert_eq!(expected, b1);
    }

    #[test]
    fn test_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result = parse_game(input).expect("Invalid input");

        let expected = CubeConundrum(
            1,
            vec![Bag::new(4, 0, 3), Bag::new(1, 2, 6), Bag::new(0, 2, 0)],
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn test_min_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = parse_game(input).expect("Invalid input");

        let bag = game.minimum_bag();

        let expected = Bag::new(4, 2, 6);

        assert_eq!(expected, bag);
    }
}
