use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (guard, x_lookup, y_lookup) = day_06::parser::parse(input)?;

    let path = day_06::get_out(&x_lookup, &y_lookup, guard).context("Guard enters a loop")?;

    let visited = day_06::collapse_directions(path);

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
