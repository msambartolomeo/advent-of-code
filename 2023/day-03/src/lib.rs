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
    pub fn new(id: u64, x: usize, y: usize) -> Self {
        let lenght = id.to_string().len();

        if id == 848 {
            dbg!(x, y);
        }

        assert!(x >= lenght, "lenght {lenght}, y {x}");

        let positions = (x - lenght..x).map(|x| Position { x, y }).collect();

        Self { id, positions }
    }

    pub fn is_valid(&self, symbols: &[Symbol]) -> bool {
        self.positions
            .iter()
            .any(|p| symbols.iter().any(|s| p.contact(&s.position)))
    }
}

impl Symbol {
    pub fn new(c: char, x: usize, y: usize) -> Self {
        Self {
            symbol: c,
            position: Position { x, y },
        }
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
        return false;
    }
}

pub fn parse_schematic(schematic: &str) -> Result<Schematic> {
    let mut symbols: Vec<Symbol> = vec![];
    let mut parts: Vec<PartNumber> = vec![];

    for (y, line) in schematic.lines().enumerate() {
        let mut current_number = None;

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let n = c.to_digit(10).context("is ascii digit")? as u64;
                current_number = add_next_ten(current_number, n);
                continue;
            } else {
                match current_number.take() {
                    Some(n) => {
                        parts.push(PartNumber::new(n, x, y));
                    }
                    None => (),
                };
            }

            if c != '.' {
                symbols.push(Symbol::new(c, x, y));
            }
        }

        match current_number.take() {
            Some(n) => {
                parts.push(PartNumber::new(n, line.len(), y));
            }
            None => (),
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
