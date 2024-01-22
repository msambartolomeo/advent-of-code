use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Not};

use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy)]
pub struct Splitter {
    splitter_type: SplitterType,
    energized: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SplitterOutput {
    Split([Direction; 2]),
    Continue,
}

impl Splitter {
    const fn new(splitter_type: SplitterType) -> Self {
        Self {
            splitter_type,
            energized: false,
        }
    }

    pub fn split(&mut self, direction: Direction) -> Option<SplitterOutput> {
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

#[derive(Debug, Clone, Copy)]
pub enum SplitterType {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
pub struct Mirror {
    mirror_type: MirrorType,
    left_energized: bool,
    right_energized: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum MirrorType {
    Left,
    Right,
}

impl Mirror {
    const fn new(mirror_type: MirrorType) -> Self {
        Self {
            mirror_type,
            left_energized: false,
            right_energized: false,
        }
    }

    #[must_use]
    pub const fn get_redirected_direction(&self, direction: Direction) -> Direction {
        match self.mirror_type {
            MirrorType::Left => match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            MirrorType::Right => match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        }
    }

    pub fn redirect(&mut self, direction: Direction) -> Option<Direction> {
        let new_direction = self.get_redirected_direction(direction);

        match (self.mirror_type, direction) {
            (MirrorType::Left, Direction::Right | Direction::Down)
            | (MirrorType::Right, Direction::Right | Direction::Up) => {
                self.left_energized.not().then(|| {
                    self.left_energized = true;
                    new_direction
                })
            }
            (MirrorType::Left, Direction::Left | Direction::Up)
            | (MirrorType::Right, Direction::Left | Direction::Down) => {
                self.right_energized.not().then(|| {
                    self.right_energized = true;
                    new_direction
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LightDeflector {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl TryFrom<char> for LightDeflector {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '/' => Ok(Self::Mirror(Mirror::new(MirrorType::Left))),
            '\\' => Ok(Self::Mirror(Mirror::new(MirrorType::Right))),
            '-' => Ok(Self::Splitter(Splitter::new(SplitterType::Horizontal))),
            '|' => Ok(Self::Splitter(Splitter::new(SplitterType::Vertical))),
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

#[derive(Debug, Clone)]
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
    #[must_use]
    pub fn next_position(
        &self,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => Some((x, y.checked_sub(1)?)),
            Direction::Down => (y != self.height - 1).then_some((x, y + 1)),
            Direction::Left => Some((x.checked_sub(1)?, y)),
            Direction::Right => (x != self.length - 1).then_some((x + 1, y)),
        }
    }

    pub fn energize(
        &mut self,
        direction: Direction,
        position: Option<(usize, usize)>,
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        position.map_or_else(
            || Box::new(std::iter::empty()) as Box<dyn Iterator<Item = _>>,
            |position| match self.get_mut(&position) {
                Some(deflector) => match deflector {
                    LightDeflector::Mirror(mirror) => mirror.redirect(direction).map_or_else(
                        || Box::new(std::iter::empty()) as Box<dyn Iterator<Item = _>>,
                        |new_direction| self.energize_next(new_direction, position),
                    ),
                    LightDeflector::Splitter(splitter) => match splitter.split(direction) {
                        Some(SplitterOutput::Split([d1, d2])) => {
                            let p1 = self.next_position(position, d1);
                            let p2 = self.next_position(position, d2);

                            let it = self.energize(d1, p1).chain(self.energize(d2, p2));

                            Box::new(std::iter::once(position).chain(it))
                        }
                        Some(SplitterOutput::Continue) => self.energize_next(direction, position),
                        None => Box::new(std::iter::empty()),
                    },
                },
                None => self.energize_next(direction, position),
            },
        )
    }

    #[inline]
    fn energize_next(
        &mut self,
        direction: Direction,
        position: (usize, usize),
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let new_position = self.next_position(position, direction);
        let it = self.energize(direction, new_position);

        Box::new(std::iter::once(position).chain(it))
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
