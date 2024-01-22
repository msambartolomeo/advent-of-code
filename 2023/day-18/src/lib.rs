use std::str::FromStr;

use anyhow::{bail, Context, Ok, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "U" | "3" => Ok(Self::Up),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            "R" | "0" => Ok(Self::Right),
            _ => bail!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub meters: u64,
    pub direction: Direction,
}

#[allow(clippy::cast_possible_truncation)]
impl FromStr for RGB {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (meters, direction) = s[1..].split_at(5);
        let meters = u64::from(
            meters
                .chars()
                .rev()
                .enumerate()
                .try_fold(0, |acc, (i, c)| {
                    Some(acc + c.to_digit(16)? * 16u32.pow(i as u32))
                })
                .context("Meters must be a 5 digit hex value")?,
        );
        let direction = direction.parse()?;

        Ok(Self { meters, direction })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DigInstruction {
    pub direction: Direction,
    pub meters: u64,
    pub color: RGB,
}

#[derive(Debug, Clone, Copy, Default)]
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
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub const fn get_next(&self, direction: Direction, meters: u64) -> Self {
        let mut position = *self;
        let meters = meters as i64;

        match direction {
            Direction::Up => position.y -= meters,
            Direction::Down => position.y += meters,
            Direction::Left => position.x -= meters,
            Direction::Right => position.x += meters,
        }

        position
    }
}

#[derive(Debug, Default)]
pub struct ShoeLacePick {
    value: i64,
    border_count: u64,
    last_vertex: Position,
}

impl ShoeLacePick {
    #[must_use]
    pub const fn last_vertex(&self) -> Position {
        self.last_vertex
    }

    pub fn add_vertex(&mut self, next: Position, border_between_vertices: u64) {
        let last = self.last_vertex;
        self.value += last.x * next.y - last.y * next.x;

        self.border_count += border_between_vertices;

        self.last_vertex = next;
    }

    #[must_use]
    pub const fn finish(self) -> u64 {
        let area = (self.value / 2).unsigned_abs();

        let internal = area + 1 - self.border_count / 2;

        self.border_count + internal
    }
}

pub fn parse_dig_plan(input: &str) -> impl Iterator<Item = Result<DigInstruction>> + '_ {
    input.lines().map(parse_dig_instruction)
}

fn parse_dig_instruction(input: &str) -> Result<DigInstruction> {
    let (instruction,) = input
        .split_whitespace()
        .tuples()
        .map(|(direction, meters, color)| {
            let direction = direction.parse()?;
            let meters = meters.parse()?;
            let color = color[1..color.len() - 1].parse()?;

            Ok(DigInstruction {
                direction,
                meters,
                color,
            })
        })
        .collect_tuple()
        .context("There must be only one instruction")?;

    instruction
}
