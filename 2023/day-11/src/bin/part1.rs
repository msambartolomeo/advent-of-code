use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<usize> {
    let mut galaxies = day_11::parse_cosmos(input).collect_vec();

    day_11::expand_galaxy(&mut galaxies, 2);

    let result = galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.x().abs_diff(g2.x()) + g1.y().abs_diff(g2.y()))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = process(input).unwrap();

        assert_eq!(374, result);
    }
}
