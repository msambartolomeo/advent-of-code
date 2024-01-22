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
    pub const fn contrary(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn move_position(self, (x, y): Position) -> Option<Position> {
        Some(match self {
            Self::North => (x, y.checked_sub(1)?),
            Self::South => (x, y.checked_add(1)?),
            Self::East => (x.checked_add(1)?, y),
            Self::West => (x.checked_sub(1)?, y),
        })
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub const fn openings(self) -> [Coordinate; 2] {
        match self {
            Self::Vertical => [Coordinate::North, Coordinate::South],
            Self::Horizontal => [Coordinate::East, Coordinate::West],
            Self::BendNE => [Coordinate::North, Coordinate::East],
            Self::BendNW => [Coordinate::North, Coordinate::West],
            Self::BendSE => [Coordinate::South, Coordinate::East],
            Self::BendSW => [Coordinate::South, Coordinate::West],
        }
    }

    #[must_use]
    pub fn get_exit(self, entrance: &Coordinate) -> Option<Coordinate> {
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
                Ok(Self::Vertical)
            }
            (Coordinate::East, Coordinate::West) | (Coordinate::West, Coordinate::East) => {
                Ok(Self::Horizontal)
            }
            (Coordinate::North, Coordinate::East) | (Coordinate::East, Coordinate::North) => {
                Ok(Self::BendNE)
            }
            (Coordinate::North, Coordinate::West) | (Coordinate::West, Coordinate::North) => {
                Ok(Self::BendNW)
            }
            (Coordinate::South, Coordinate::East) | (Coordinate::East, Coordinate::South) => {
                Ok(Self::BendSE)
            }
            (Coordinate::South, Coordinate::West) | (Coordinate::West, Coordinate::South) => {
                Ok(Self::BendSW)
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
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::BendNE),
            'J' => Ok(Self::BendNW),
            '7' => Ok(Self::BendSW),
            'F' => Ok(Self::BendSE),
            'S' => Err(InvalidPipe::Start),
            '.' => Err(InvalidPipe::None),
            _ => Err(InvalidPipe::Error),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => f.write_char('|'),
            Self::Horizontal => f.write_char('-'),
            Self::BendNE => f.write_char('L'),
            Self::BendNW => f.write_char('J'),
            Self::BendSE => f.write_char('F'),
            Self::BendSW => f.write_char('7'),
        }
    }
}

type Position = (usize, usize);

pub struct Pipes {
    pub matrix: Vec<Vec<Option<Pipe>>>,
    length: usize,
    height: usize,
}

impl TryFrom<Vec<Vec<Option<Pipe>>>> for Pipes {
    type Error = anyhow::Error;

    fn try_from(matrix: Vec<Vec<Option<Pipe>>>) -> Result<Self> {
        let height = matrix.len();
        let length = matrix.first().context("Must not be empty")?.len();

        Ok(Self {
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

    /// # Precondition
    /// The pipes matrix must have a loop and the start position must be part of that loop
    ///
    /// # Panics
    /// The starting position must belong to a pipe
    pub fn pipe_loop(&self, start: Position) -> impl Iterator<Item = Position> + '_ {
        let pipe = self.get(start).expect("Start position must exist");
        let mut position = start;
        let mut coordinate = pipe.openings()[0];

        std::iter::once(start).chain(std::iter::from_fn(move || {
            (position, coordinate) = self.get_next(position, coordinate)?;
            (position != start).then_some(position)
        }))
    }

    #[must_use]
    pub fn pipe_loop_matrix(&self, start: Position) -> Vec<Vec<bool>> {
        let mut matrix = vec![vec![false; self.length]; self.height];

        for (x, y) in self.pipe_loop(start) {
            matrix[y][x] = true;
        }

        matrix
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

/// # Errors
/// If the pipe map has incorrect pipes
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
