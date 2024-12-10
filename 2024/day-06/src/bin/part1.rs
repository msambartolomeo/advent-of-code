use std::collections::HashSet;

use anyhow::Result;
use day_06::Direction;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (mut guard, x_lookup, y_lookup, width, height) = day_06::parser::parse(input)?;

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
                Direction::South => guard.advance(height),
                Direction::East => guard.advance(width),
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

    let result = visited.len() as u64;

    Ok(result)
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
