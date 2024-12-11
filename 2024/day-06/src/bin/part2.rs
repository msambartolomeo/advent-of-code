use anyhow::{Context, Result};
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

    let path = day_06::get_out(&x_lookup, &y_lookup, guard)
        .context("Guard enters a loop in original path")?;

    let mut visited = day_06::collapse_directions(path);

    visited.remove(&guard.position());

    let result = visited
        .into_par_iter()
        .filter(|&(x, y)| {
            let x_lookup = x_lookup.new_with(y, x);
            let y_lookup = y_lookup.new_with(x, y);

            day_06::get_out(&x_lookup, &y_lookup, guard).is_none()
        })
        .count() as u64;

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
        let expected: u64 = 6;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
