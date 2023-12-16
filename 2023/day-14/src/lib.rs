use std::{
    collections::HashMap,
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
