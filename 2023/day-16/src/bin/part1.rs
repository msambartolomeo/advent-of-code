use std::collections::HashSet;

use anyhow::Result;
use day_16::Direction;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut contraption = day_16::parse_contraption(input)?;

    let result = day_16::energize(&mut contraption, Direction::Right, Some((0, 0)))
        .collect::<HashSet<_>>()
        .len() as u64;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

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

        assert_eq!(46, result);

        Ok(())
    }
}
