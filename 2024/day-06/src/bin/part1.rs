use std::collections::HashSet;

use anyhow::Result;
use day_06::{Direction, Guard, Lookup};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (guard, x_lookup, y_lookup) = day_06::parser::parse(input)?;

    let result = get_out(&x_lookup, &y_lookup, guard);

    Ok(result)
}

#[must_use]
pub fn get_out(x_lookup: &Lookup, y_lookup: &Lookup, mut guard: Guard) -> u64 {
    let mut visited = HashSet::new();

    loop {
        let obstacle = match guard.direction() {
            Direction::North => y_lookup.find_prev(guard.x(), guard.y()),
            Direction::South => y_lookup.find_next(guard.x(), guard.y()),
            Direction::East => x_lookup.find_next(guard.y(), guard.x()),
            Direction::West => x_lookup.find_prev(guard.y(), guard.x()),
        };

        let Some(obstacle) = obstacle else {
            let range = match guard.direction() {
                Direction::North | Direction::West => guard.advance(0),
                Direction::South => guard.advance(x_lookup.len()),
                Direction::East => guard.advance(y_lookup.len()),
            };

            for p in range {
                visited.insert(p);
            }

            break;
        };

        for p in guard.advance(obstacle) {
            visited.insert(p);
        }
    }

    visited.len() as u64
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 41;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
