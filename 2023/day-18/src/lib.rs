use std::str::FromStr;

use anyhow::{bail, ensure, Context, Ok, Result};
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
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => bail!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(clippy::cast_possible_truncation)]
impl FromStr for RGB {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        ensure!(chars.next() == Some('#'));

        let (r, g, b) = chars
            .tuples()
            .filter_map(|(n, m)| Some(n.to_digit(16)? as u8 * 16 + m.to_digit(16)? as u8))
            .collect_tuple()
            .context("6 digits base 16 must be provided")?;

        Ok(RGB { r, g, b })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DigInstruction {
    pub direction: Direction,
    pub meters: u32,
    pub color: RGB,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

impl Position {
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn get_next(&self, direction: Direction, meters: u32) -> Self {
        let mut position = *self;
        let meters = meters as i32;

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
    value: i32,
    border_count: u32,
    last_vertex: Position,
}

impl ShoeLacePick {
    #[must_use]
    pub fn last_vertex(&self) -> Position {
        self.last_vertex
    }

    pub fn add_vertex(&mut self, next: Position, border_between_vertices: u32) {
        let last = self.last_vertex;
        self.value += last.x * next.y - last.y * next.x;

        self.border_count += border_between_vertices;

        self.last_vertex = next;
    }

    #[must_use]
    pub fn finish(self) -> u32 {
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
            let color = color[1..].parse()?;

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
