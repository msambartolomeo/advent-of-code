use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Not};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub struct Splitter {
    splitter_type: SplitterType,
    energized: bool,
}

#[derive(Debug)]
pub enum SplitterOutput {
    Split([Direction; 2]),
    Continue,
}

impl Splitter {
    fn new(splitter_type: SplitterType) -> Self {
        Self {
            splitter_type,
            energized: false,
        }
    }

    pub fn get_output(&mut self, direction: Direction) -> Option<SplitterOutput> {
        match self.splitter_type {
            SplitterType::Vertical => match direction {
                Direction::Left | Direction::Right => self.energized.not().then(|| {
                    self.energized = true;
                    SplitterOutput::Split([Direction::Up, Direction::Down])
                }),
                Direction::Up | Direction::Down => Some(SplitterOutput::Continue),
            },
            SplitterType::Horizontal => match direction {
                Direction::Up | Direction::Down => self.energized.not().then(|| {
                    self.energized = true;
                    SplitterOutput::Split([Direction::Left, Direction::Right])
                }),
                Direction::Left | Direction::Right => Some(SplitterOutput::Continue),
            },
        }
    }
}

#[derive(Debug)]
pub enum SplitterType {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Mirror {
    mirror_type: MirrorType,
    left_energized: bool,
    right_energized: bool,
}

impl Mirror {
    fn new(mirror_type: MirrorType) -> Self {
        Self {
            mirror_type,
            left_energized: false,
            right_energized: false,
        }
    }

    pub fn get_output(&mut self, direction: Direction) -> Option<Direction> {
        match self.mirror_type {
            MirrorType::Left => match direction {
                Direction::Right => self.left_energized.not().then(|| {
                    self.left_energized = true;
                    Direction::Up
                }),
                Direction::Down => self.left_energized.not().then(|| {
                    self.left_energized = true;
                    Direction::Left
                }),
                Direction::Left => self.right_energized.not().then(|| {
                    self.right_energized = true;
                    Direction::Down
                }),
                Direction::Up => self.right_energized.not().then(|| {
                    self.right_energized = true;
                    Direction::Right
                }),
            },
            MirrorType::Right => match direction {
                Direction::Down => self.left_energized.not().then(|| {
                    self.left_energized = true;
                    Direction::Right
                }),
                Direction::Left => self.left_energized.not().then(|| {
                    self.left_energized = true;
                    Direction::Up
                }),
                Direction::Right => self.right_energized.not().then(|| {
                    self.right_energized = true;
                    Direction::Down
                }),
                Direction::Up => self.right_energized.not().then(|| {
                    self.right_energized = true;
                    Direction::Left
                }),
            },
        }
    }
}

#[derive(Debug)]
pub enum MirrorType {
    Left,
    Right,
}

#[derive(Debug)]
pub enum LightDeflector {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl TryFrom<char> for LightDeflector {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '/' => Ok(LightDeflector::Mirror(Mirror::new(MirrorType::Left))),
            '\\' => Ok(LightDeflector::Mirror(Mirror::new(MirrorType::Right))),
            '-' => Ok(LightDeflector::Splitter(Splitter::new(
                SplitterType::Horizontal,
            ))),
            '|' => Ok(LightDeflector::Splitter(Splitter::new(
                SplitterType::Vertical,
            ))),
            _ => bail!("Invalid light deflector Type"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coordinates = (usize, usize);

#[derive(Debug)]
pub struct Contraption {
    pub matrix: HashMap<Coordinates, LightDeflector>,
    pub length: usize,
    pub height: usize,
}

impl Deref for Contraption {
    type Target = HashMap<Coordinates, LightDeflector>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl DerefMut for Contraption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}

impl Contraption {
    pub fn next_position(
        &self,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => (y != 0).then(|| (x, y - 1)),
            Direction::Down => (y != self.height - 1).then(|| (x, y + 1)),
            Direction::Left => (x != 0).then(|| (x - 1, y)),
            Direction::Right => (x != self.length - 1).then(|| (x + 1, y)),
        }
    }
}

/// Parses a contraption
///
/// # Errors
/// If the input is not valid
pub fn parse_contraption(input: &str) -> Result<Contraption> {
    let height = input.lines().count();
    let length = input.lines().next().context("Input not empty")?.len();

    let matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                (c != '.').then_some(TryInto::<LightDeflector>::try_into(c).map(|r| ((x, y), r)))
            })
        })
        .collect::<Result<_>>()?;

    Ok(Contraption {
        matrix,
        length,
        height,
    })
}
