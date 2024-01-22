use std::{collections::HashSet, num::TryFromIntError};

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Schematic {
    pub parts: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug)]
pub struct PartNumber {
    pub id: u64,
    pub positions: Vec<Position>,
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol: char,
    pub position: Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl TryFrom<(i128, i128)> for Position {
    type Error = TryFromIntError;

    fn try_from((x, y): (i128, i128)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl PartNumber {
    fn new(id: u64, x: usize, y: usize) -> Self {
        let lenght = id.to_string().len();

        assert!(x >= lenght, "lenght {lenght}, y {x}");

        let positions = (x - lenght..x).map(|x| (x, y).into()).collect();

        Self { id, positions }
    }

    #[must_use]
    pub fn is_valid(&self, symbols: &[Symbol]) -> bool {
        let adj = adjacent_positions(&self.positions);

        symbols.iter().any(|s| adj.contains(&s.position))
    }
}

impl Symbol {
    fn new(c: char, x: usize, y: usize) -> Self {
        Self {
            symbol: c,
            position: (x, y).into(),
        }
    }

    #[must_use]
    pub fn gear_ratio(&self, parts: &[PartNumber]) -> Option<u64> {
        if self.symbol != '*' {
            // NOTE: if symbol is not *, its not a gear
            return None;
        }

        let adj = adjacent_positions(&[self.position]);

        let contact_parts: Vec<u64> = parts
            .iter()
            .filter_map(|part| {
                if part.positions.iter().any(|p| adj.contains(p)) {
                    Some(part.id)
                } else {
                    None
                }
            })
            .collect();

        if contact_parts.len() != 2 {
            // NOTE: if not two connecting parts, its not a gear
            return None;
        }

        Some(contact_parts.iter().product())
    }
}

#[must_use]
fn adjacent_positions(positions: &[Position]) -> HashSet<Position> {
    let west_idx = 0;
    let east_idx = positions.len() - 1;

    let mut set: HashSet<Position> = HashSet::with_capacity(6 + 2 * positions.len());

    for (idx, position) in positions.iter().enumerate() {
        // NOTE: Cast positions as signed to prevent panics
        let (x, y) = (position.x as i128, position.y as i128);

        // NOTE: In every position add north and south borders
        set.extend(
            [(x, y + 1), (x, y - 1)]
                .into_iter()
                .filter_map(|p| TryInto::<Position>::try_into(p).ok()),
        );

        // NOTE: On the first position add west border
        if idx == west_idx {
            set.extend(
                [(x - 1, y), (x - 1, y + 1), (x - 1, y - 1)]
                    .into_iter()
                    .filter_map(|p| TryInto::<Position>::try_into(p).ok()),
            );
        };

        // NOTE: On the last position add east border
        if idx == east_idx {
            set.extend(
                [(x + 1, y), (x + 1, y + 1), (x + 1, y - 1)]
                    .into_iter()
                    .filter_map(|p| TryInto::<Position>::try_into(p).ok()),
            );
        }
    }

    set
}

/// Parses schematic for the gondola lift
///
/// # Errors
///
/// Returns error if ther is a parsing error
pub fn parse_schematic(schematic: &str) -> Result<Schematic> {
    let mut symbols: Vec<Symbol> = vec![];
    let mut parts: Vec<PartNumber> = vec![];

    for (y, line) in schematic.lines().enumerate() {
        let mut current_number = None;

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let n = u64::from(c.to_digit(10).context("is ascii digit")?);
                current_number.replace(add_next_ten(current_number, n));
                continue;
            }

            if let Some(n) = current_number.take() {
                parts.push(PartNumber::new(n, x, y));
            };

            if c != '.' {
                symbols.push(Symbol::new(c, x, y));
            }
        }

        if let Some(n) = current_number.take() {
            parts.push(PartNumber::new(n, line.len(), y));
        };
    }

    Ok(Schematic { parts, symbols })
}

#[must_use]
fn add_next_ten(n: Option<u64>, m: u64) -> u64 {
    n.map_or(m, |n| n * 10 + m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_next_ten() {
        let mut n = Some(4);

        let m = 6;

        n.replace(add_next_ten(n, m));

        let m = 7;

        let result = add_next_ten(n, m);

        assert_eq!(467, result);
    }
}
