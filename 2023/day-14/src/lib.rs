use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::ops::{Deref, DerefMut};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub enum Rock {
    Rounded,
    Cube,
}

impl TryFrom<char> for Rock {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'O' => Ok(Rock::Rounded),
            '#' => Ok(Rock::Cube),
            _ => bail!("Invalid rock"),
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Rounded => f.write_char('O'),
            Rock::Cube => f.write_char('#'),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: u64,
    pub y: u64,
}

impl From<(usize, usize)> for Coordinates {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as u64,
            y: y as u64,
        }
    }
}

pub struct Platform {
    pub matrix: HashMap<Coordinates, Rock>,
    pub length: usize,
    pub height: usize,
}

impl Deref for Platform {
    type Target = HashMap<Coordinates, Rock>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.length {
                match self.get(&(x, y).into()) {
                    Some(rock) => f.write_str(&rock.to_string())?,
                    None => f.write_char('.')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn slide_platform_cycle(platform: &mut Platform) {
    slide_platform_north(platform);
    slide_platform_west(platform);
    slide_platform_south(platform);
    slide_platform_east(platform);
}

pub fn slide_platform_north(platform: &mut Platform) {
    for x in 0..platform.length {
        let mut rock_to_move = None;
        for y in (0..platform.height).rev() {
            match platform.get(&(x, y).into()) {
                Some(Rock::Rounded) => {
                    if rock_to_move.is_none() {
                        rock_to_move = Some(y);
                    }
                }
                Some(Rock::Cube) => rock_to_move = None,
                None => {
                    if let Some(old_rock) = rock_to_move.take() {
                        platform.remove(&(x, old_rock).into());
                        platform.insert((x, y).into(), Rock::Rounded);

                        rock_to_move = Some(old_rock - 1);
                    }
                }
            }
        }
    }
}

pub fn slide_platform_west(platform: &mut Platform) {
    for y in 0..platform.height {
        let mut rock_to_move = None;
        for x in (0..platform.length).rev() {
            match platform.get(&(x, y).into()) {
                Some(Rock::Rounded) => {
                    if rock_to_move.is_none() {
                        rock_to_move = Some(x);
                    }
                }
                Some(Rock::Cube) => rock_to_move = None,
                None => {
                    if let Some(old_rock) = rock_to_move.take() {
                        platform.remove(&(old_rock, y).into());
                        platform.insert((x, y).into(), Rock::Rounded);

                        rock_to_move = Some(old_rock - 1);
                    }
                }
            }
        }
    }
}

pub fn slide_platform_south(platform: &mut Platform) {
    for x in 0..platform.length {
        let mut rock_to_move = None;
        for y in 0..platform.height {
            match platform.get(&(x, y).into()) {
                Some(Rock::Rounded) => {
                    if rock_to_move.is_none() {
                        rock_to_move = Some(y);
                    }
                }
                Some(Rock::Cube) => rock_to_move = None,
                None => {
                    if let Some(old_rock) = rock_to_move.take() {
                        platform.remove(&(x, old_rock).into());
                        platform.insert((x, y).into(), Rock::Rounded);

                        rock_to_move = Some(old_rock + 1);
                    }
                }
            }
        }
    }
}

pub fn slide_platform_east(platform: &mut Platform) {
    for y in 0..platform.height {
        let mut rock_to_move = None;
        for x in 0..platform.length {
            match platform.get(&(x, y).into()) {
                Some(Rock::Rounded) => {
                    if rock_to_move.is_none() {
                        rock_to_move = Some(x);
                    }
                }
                Some(Rock::Cube) => rock_to_move = None,
                None => {
                    if let Some(old_rock) = rock_to_move.take() {
                        platform.remove(&(old_rock, y).into());
                        platform.insert((x, y).into(), Rock::Rounded);

                        rock_to_move = Some(old_rock + 1);
                    }
                }
            }
        }
    }
}

/// Parses a platform
///
/// # Errors
/// If the input is not valid
pub fn parse_platform(input: &str) -> Result<Platform> {
    let height = input.lines().count();
    let length = input.lines().next().context("Input not empty")?.len();

    let matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                (c != '.').then_some(TryInto::<Rock>::try_into(c).map(|r| ((x, y).into(), r)))
            })
        })
        .collect::<Result<_>>()?;

    Ok(Platform {
        matrix,
        length,
        height,
    })
}
