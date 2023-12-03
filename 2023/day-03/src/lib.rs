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

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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
        self.positions
            .iter()
            .any(|p| symbols.iter().any(|s| p.contact(&s.position)))
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

        let contact_parts: Vec<u64> = parts
            .iter()
            .filter(|part| part.positions.iter().any(|p| p.contact(&self.position)))
            .map(|p| p.id)
            .collect();

        if contact_parts.len() != 2 {
            // NOTE: if not two connecting parts, its not a gear
            return None;
        }

        Some(contact_parts.iter().product())
    }
}

impl Position {
    fn contact(&self, other: &Self) -> bool {
        let (x, y) = (self.x, self.y);
        let other = (other.x, other.y);

        if [(x, y + 1), (x + 1, y), (x + 1, y + 1)].contains(&other) {
            return true;
        }

        if x > 0 && [(x - 1, y), (x - 1, y + 1)].contains(&other) {
            return true;
        }

        if y > 0 && [(x, y - 1), (x + 1, y - 1)].contains(&other) {
            return true;
        }

        if x > 0 && y > 0 && (x - 1, y - 1) == other {
            return true;
        }
        false
    }
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
                current_number = add_next_ten(current_number, n);
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

fn add_next_ten(n: Option<u64>, m: u64) -> Option<u64> {
    match n {
        Some(n) => Some(n * 10 + m),
        None => Some(m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_next_ten() {
        let n = Some(4);

        let m = 6;

        let n = add_next_ten(n, m);

        let m = 7;

        let result = add_next_ten(n, m);

        assert_eq!(Some(467), result);
    }
}
