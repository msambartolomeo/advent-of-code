use std::collections::HashSet;

use anyhow::{Context, Result};
use day_16::{Contraption, Direction};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let contraption = day_16::parse_contraption(input)?;

    let length = contraption.length;
    let height = contraption.height;

    let result = (0..length)
        .into_par_iter()
        .map(|i| test_ray(&contraption, Direction::Down, (i, 0)))
        .chain(
            (0..height)
                .into_par_iter()
                .map(|i| test_ray(&contraption, Direction::Right, (0, i))),
        )
        .chain(
            (0..length)
                .into_par_iter()
                .map(|i| test_ray(&contraption, Direction::Up, (i, height - 1))),
        )
        .chain(
            (0..height)
                .into_par_iter()
                .map(|i| test_ray(&contraption, Direction::Left, (length - 1, i))),
        )
        .max()
        .context("Expect at least one")?;

    Ok(result)
}

#[inline]
#[must_use]
fn test_ray(contraption: &Contraption, direction: Direction, position: (usize, usize)) -> u64 {
    contraption
        .clone()
        .energize(direction, Some(position))
        .collect::<HashSet<_>>()
        .len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        let result = process(input)?;

        assert_eq!(51, result);

        Ok(())
    }
}
