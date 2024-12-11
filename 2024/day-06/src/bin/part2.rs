use std::collections::HashSet;

use anyhow::{Context, Result};
use day_06::{Direction, Guard, Lookup};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (guard, x_lookup, y_lookup) = day_06::parser::parse(input)?;

    let original_path =
        get_out(&x_lookup, &y_lookup, guard).context("Should find original path")?;

    let mut visited = original_path
        .into_iter()
        .map(|(i, _)| i)
        .collect::<HashSet<_>>();

    visited.remove(&guard.position());

    let result = visited
        .into_par_iter()
        .filter(|&(x, y)| {
            let mut x_lookup = x_lookup.clone();
            let mut y_lookup = y_lookup.clone();

            x_lookup.add(y, x);
            y_lookup.add(x, y);

            get_out(&x_lookup, &y_lookup, guard).is_none()
        })
        .count() as u64;

    Ok(result)
}

type Visited = HashSet<((usize, usize), Direction)>;

#[must_use]
pub fn get_out(x_lookup: &Lookup, y_lookup: &Lookup, mut guard: Guard) -> Option<Visited> {
    let mut visited = HashSet::new();

    loop {
        let direction = guard.direction();

        let obstacle = match direction {
            Direction::North => y_lookup.find_prev(guard.x(), guard.y()),
            Direction::South => y_lookup.find_next(guard.x(), guard.y()),
            Direction::East => x_lookup.find_next(guard.y(), guard.x()),
            Direction::West => x_lookup.find_prev(guard.y(), guard.x()),
        };

        let Some(obstacle) = obstacle else {
            let positions = match guard.direction() {
                Direction::North | Direction::West => guard.advance(0),
                Direction::South => guard.advance(x_lookup.len()),
                Direction::East => guard.advance(y_lookup.len()),
            };

            let added = add_visited(&mut visited, positions, direction);
            if !added {
                break None;
            }

            break Some(visited);
        };

        let positions = guard.advance(obstacle);
        let added = add_visited(&mut visited, positions, direction);
        if !added {
            break None;
        }
    }
}

pub fn add_visited(
    visited: &mut Visited,
    positions: impl Iterator<Item = (usize, usize)>,
    direction: Direction,
) -> bool {
    for p in positions {
        if visited.contains(&(p, direction)) {
            return false;
        }
        visited.insert((p, direction));
    }

    return true;
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
        let expected: u64 = 6;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
