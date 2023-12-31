use std::fmt::{Display, Write};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coordinate {
    North,
    South,
    East,
    West,
}

impl Coordinate {
    #[must_use]
    pub fn contrary(self) -> Self {
        match self {
            Coordinate::North => Coordinate::South,
            Coordinate::South => Coordinate::North,
            Coordinate::East => Coordinate::West,
            Coordinate::West => Coordinate::East,
        }
    }

    fn move_position(self, (x, y): Position) -> Option<Position> {
        Some(match self {
            Coordinate::North => (x, y.checked_sub(1)?),
            Coordinate::South => (x, y.checked_add(1)?),
            Coordinate::East => (x.checked_add(1)?, y),
            Coordinate::West => (x.checked_sub(1)?, y),
        })
    }
}

#[derive(Debug)]
pub enum Pipe {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
}

impl Pipe {
    #[must_use]
    pub const fn openings(&self) -> [Coordinate; 2] {
        match self {
            Pipe::Vertical => [Coordinate::North, Coordinate::South],
            Pipe::Horizontal => [Coordinate::East, Coordinate::West],
            Pipe::BendNE => [Coordinate::North, Coordinate::East],
            Pipe::BendNW => [Coordinate::North, Coordinate::West],
            Pipe::BendSE => [Coordinate::South, Coordinate::East],
            Pipe::BendSW => [Coordinate::South, Coordinate::West],
        }
    }

    #[must_use]
    pub fn get_exit(&self, entrance: &Coordinate) -> Option<Coordinate> {
        let openings = self.openings();
        if openings.contains(entrance) {
            openings.iter().find(|&c| c != entrance).copied()
        } else {
            None
        }
    }
}

impl TryFrom<(Coordinate, Coordinate)> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: (Coordinate, Coordinate)) -> Result<Self> {
        match value {
            (Coordinate::North, Coordinate::South) | (Coordinate::South, Coordinate::North) => {
                Ok(Pipe::Vertical)
            }
            (Coordinate::East, Coordinate::West) | (Coordinate::West, Coordinate::East) => {
                Ok(Pipe::Horizontal)
            }
            (Coordinate::North, Coordinate::East) | (Coordinate::East, Coordinate::North) => {
                Ok(Pipe::BendNE)
            }
            (Coordinate::North, Coordinate::West) | (Coordinate::West, Coordinate::North) => {
                Ok(Pipe::BendNW)
            }
            (Coordinate::South, Coordinate::East) | (Coordinate::East, Coordinate::South) => {
                Ok(Pipe::BendSE)
            }
            (Coordinate::South, Coordinate::West) | (Coordinate::West, Coordinate::South) => {
                Ok(Pipe::BendSW)
            }
            _ => bail!("Repeated coordinates are invalid"),
        }
    }
}

pub enum InvalidPipe {
    None,
    Start,
    Error,
}

impl TryFrom<char> for Pipe {
    type Error = InvalidPipe;

    fn try_from(value: char) -> Result<Self, InvalidPipe> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::BendNE),
            'J' => Ok(Pipe::BendNW),
            '7' => Ok(Pipe::BendSW),
            'F' => Ok(Pipe::BendSE),
            'S' => Err(InvalidPipe::Start),
            '.' => Err(InvalidPipe::None),
            _ => Err(InvalidPipe::Error),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical => f.write_char('|'),
            Pipe::Horizontal => f.write_char('-'),
            Pipe::BendNE => f.write_char('L'),
            Pipe::BendNW => f.write_char('J'),
            Pipe::BendSE => f.write_char('F'),
            Pipe::BendSW => f.write_char('7'),
        }
    }
}

type Position = (usize, usize);

pub struct Pipes {
    matrix: Vec<Vec<Option<Pipe>>>,
    length: usize,
    height: usize,
}

impl TryFrom<Vec<Vec<Option<Pipe>>>> for Pipes {
    type Error = anyhow::Error;

    fn try_from(matrix: Vec<Vec<Option<Pipe>>>) -> Result<Self> {
        let height = matrix.len();
        let length = matrix.first().context("Must not be empty")?.len();

        Ok(Pipes {
            matrix,
            length,
            height,
        })
    }
}

impl Pipes {
    #[must_use]
    pub fn get(&self, (x, y): Position) -> Option<&Pipe> {
        if (0..self.length).contains(&x) && (0..self.height).contains(&y) {
            self.matrix[y][x].as_ref()
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_next(
        &self,
        position: Position,
        entrance: Coordinate,
    ) -> Option<(Position, Coordinate)> {
        if let Some(pipe) = self.get(position) {
            let exit = pipe.get_exit(&entrance)?;

            let next_position = exit.move_position(position)?;

            let next_entrance = exit.contrary();

            Some((next_position, next_entrance))
        } else {
            None
        }
    }
}

impl Display for Pipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for pipe in row {
                match pipe {
                    Some(pipe) => write!(f, "{pipe}")?,
                    None => f.write_char('.')?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_pipes(input: &str) -> Result<(Pipes, Position)> {
    let mut start = None;

    let matrix = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pipe = Pipe::try_from(c);

                    Ok(match pipe {
                        Ok(pipe) => Some(pipe),
                        Err(InvalidPipe::None) => None,
                        Err(InvalidPipe::Start) => {
                            start = Some((x, y));
                            None
                        }
                        Err(InvalidPipe::Error) => bail!("Invalid input"),
                    })
                })
                .collect::<Result<Vec<Option<Pipe>>>>()
        })
        .collect::<Result<Vec<Vec<Option<Pipe>>>>>()?;

    let mut pipes = Pipes::try_from(matrix)?;

    let start = start.context("Must have found start")?;
    let start_pipe = get_start_pipe(&pipes, start).context("Start pipe must form a loop")?;
    pipes.matrix[start.1][start.0].replace(start_pipe);

    Ok((pipes, start))
}

fn get_start_pipe(pipes: &Pipes, start: Position) -> Option<Pipe> {
    static COORDINATES: [Coordinate; 4] = [
        Coordinate::North,
        Coordinate::East,
        Coordinate::South,
        Coordinate::West,
    ];

    let valid_coordinates = COORDINATES
        .into_iter()
        .filter_map(|c| {
            let position = c.move_position(start)?;

            let pipe = pipes.get(position)?;

            pipe.openings().contains(&c.contrary()).then_some(c)
        })
        .collect_tuple::<(_, _)>()?;

    Pipe::try_from(valid_coordinates).ok()
}
