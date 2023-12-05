use std::collections::HashSet;

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
        Position { x, y }
    }
}

impl PartNumber {
    fn new(id: u64, x: usize, y: usize) -> Self {
        let lenght = id.to_string().len();

        assert!(x >= lenght, "lenght {lenght}, y {x}");

        let positions = (x - lenght..x).map(|x| Position { x, y }).collect();

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
            position: Position { x, y },
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
    let position_count = positions.len();

    let mut set: HashSet<Position> = HashSet::with_capacity(6 + 2 * position_count);

    for (idx, position) in positions.iter().enumerate() {
        let (x, y) = (position.x, position.y);

        set.insert((x, y + 1).into());

        if y > 0 {
            set.insert((x, y - 1).into());
        }

        if idx == 0 && x > 0 {
            set.extend([(x - 1, y).into(), (x - 1, y + 1).into()].iter());
            if y > 0 {
                set.insert((x - 1, y - 1).into());
            }
        };

        if idx == position_count - 1 {
            set.extend([(x + 1, y).into(), (x + 1, y + 1).into()].iter());
            if y > 0 {
                set.insert((x + 1, y - 1).into());
            }
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
                _ = current_number.insert(add_next_ten(current_number, n));
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
    match n {
        Some(n) => n * 10 + m,
        None => m,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_next_ten() {
        let mut n = Some(4);

        let m = 6;

        _ = n.insert(add_next_ten(n, m));

        let m = 7;

        let result = add_next_ten(n, m);

        assert_eq!(467, result);
    }
}
