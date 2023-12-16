use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
};

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
            f.write_char('\n')?
        }
        Ok(())
    }
}

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
