use std::collections::HashSet;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Feature {
    Plot,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
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
    infinite: bool,
}

impl Garden {
    #[must_use]
    pub fn new(matrix: Vec<Feature>, height: usize, length: usize, start: Position) -> Self {
        Garden {
            height,
            length,
            matrix,
            start,
            infinite: false,
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

        let (x, y) = if self.infinite {
            (
                usize::try_from(x.rem_euclid(i64::try_from(self.length).ok()?)).ok()?,
                usize::try_from(y.rem_euclid(i64::try_from(self.height).ok()?)).ok()?,
            )
        } else {
            (usize::try_from(x).ok()?, usize::try_from(y).ok()?)
        };

        (x < self.length && y < self.height).then(|| self.matrix[y * self.height + x])
    }

    pub fn make_infinite(&mut self) {
        self.infinite = true;
    }
}

#[derive(Debug)]
struct Switcher<T> {
    v1: T,
    v2: T,
    current: *mut T,
    other: *mut T,
}

impl<T: Default> Default for Switcher<T> {
    fn default() -> Self {
        Self {
            v1: Default::default(),
            v2: Default::default(),
            current: std::ptr::null_mut(),
            other: std::ptr::null_mut(),
        }
    }
}

impl<T> Switcher<T> {
    fn init(&mut self) {
        self.current = &mut self.v1;
        self.other = &mut self.v2;
    }

    fn is_init(&mut self) -> bool {
        (self.current == &mut self.v1 || self.current == &mut self.v2)
            && (self.other == &mut self.v1 || self.other == &mut self.v2)
    }

    #[must_use]
    fn get_current(&mut self) -> &mut T {
        assert!(self.is_init(), "Must initialize Switcher");

        unsafe { &mut *self.current }
    }

    fn switch(&mut self) -> &mut T {
        std::mem::swap(&mut self.current, &mut self.other);

        self.get_current()
    }
}

#[must_use]
pub fn random_walk_posibilities(garden: &Garden, steps: u32) -> u64 {
    let mut frontier = HashSet::from([garden.start()]);
    let mut last_frontier = HashSet::default();
    let mut switcher = Switcher::<u64>::default();
    switcher.init();

    for _ in 0..steps {
        let new_frontier = frontier
            .iter()
            .flat_map(|&pos| {
                pos.next().filter_map(|p| match garden.get(p)? {
                    Feature::Plot => {
                        (!frontier.contains(&p) && !last_frontier.contains(&p)).then_some(p)
                    }
                    Feature::Rock => None,
                })
            })
            .collect::<HashSet<_>>();

        *switcher.get_current() += frontier.len() as u64;

        last_frontier = frontier;
        frontier = new_frontier;

        switcher.switch();
    }

    *switcher.get_current() + frontier.len() as u64
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
                        start = Some(Position::from((i64::try_from(x)?, i64::try_from(y)?)));
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
