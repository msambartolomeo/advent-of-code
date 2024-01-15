use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Feature {
    Plot,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Position {
    pub fn next(self) -> impl Iterator<Item = Position> {
        let Position { x, y } = self;

        [
            (x + 1, y).into(),
            (x - 1, y).into(),
            (x, y + 1).into(),
            (x, y - 1).into(),
        ]
        .into_iter()
    }
}

#[derive(Debug)]
pub struct Garden {
    height: usize,
    length: usize,
    matrix: Vec<Feature>,
    start: Position,
}

impl Garden {
    #[must_use]
    pub fn new(matrix: Vec<Feature>, height: usize, length: usize, start: Position) -> Self {
        Garden {
            height,
            length,
            matrix,
            start,
        }
    }

    #[inline]
    #[must_use]
    pub fn start(&self) -> Position {
        self.start
    }

    #[must_use]
    pub fn get(&self, position: Position) -> Option<Feature> {
        let Position { x, y } = position;
        (x < self.length && y < self.height).then_some(self.matrix[y * self.height + x])
    }
}

/// Parses the garden plots and rocks
/// # Errors
/// If there is an invalid garden feature
pub fn parse_garden(input: &str) -> Result<Garden> {
    let height = input.lines().count();
    let length = input.len() / height - 1;

    let mut start = None;

    let matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            let vec = l
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Ok(Feature::Rock),
                    '.' => Ok(Feature::Plot),
                    'S' => {
                        start = Some(Position::from((x, y)));
                        Ok(Feature::Plot)
                    }

                    _ => bail!("Invalid feature on ({x},{y})"),
                })
                .collect_vec();
            vec
        })
        .collect::<Result<Vec<_>>>()?;

    let start = start.context("Start has been found")?;

    Ok(Garden::new(matrix, height, length, start))
}
