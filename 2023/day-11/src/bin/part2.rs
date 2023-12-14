use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input, 1000000)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str, galaxy_expansion: usize) -> Result<usize> {
    let mut galaxies = day_11::parse_cosmos(input).collect_vec();

    day_11::expand_galaxy(&mut galaxies, galaxy_expansion);

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
    fn test_10() {
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

        let result = process(input, 10).unwrap();

        assert_eq!(1030, result);
    }

    #[test]
    fn test_100() {
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

        let result = process(input, 100).unwrap();

        assert_eq!(8410, result);
    }
}
