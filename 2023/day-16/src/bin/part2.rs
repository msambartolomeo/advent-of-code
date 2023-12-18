use std::collections::HashSet;

use anyhow::{Context, Result};
use day_16::Direction;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let contraption = day_16::parse_contraption(input)?;

    let result = (0..contraption.length)
        .map(|i| {
            contraption
                .clone()
                .energize(Direction::Down, Some((i, 0)))
                .collect::<HashSet<_>>()
                .len() as u64
        })
        .chain((0..contraption.length).map(|i| {
            contraption
                .clone()
                .energize(Direction::Up, Some((i, contraption.height - 1)))
                .collect::<HashSet<_>>()
                .len() as u64
        }))
        .chain((0..contraption.height).map(|i| {
            contraption
                .clone()
                .energize(Direction::Right, Some((0, i)))
                .collect::<HashSet<_>>()
                .len() as u64
        }))
        .chain((0..contraption.height).map(|i| {
            contraption
                .clone()
                .energize(Direction::Left, Some((contraption.length - 1, i)))
                .collect::<HashSet<_>>()
                .len() as u64
        }))
        .max()
        .context("Expect at least one")?;

    Ok(result)
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
