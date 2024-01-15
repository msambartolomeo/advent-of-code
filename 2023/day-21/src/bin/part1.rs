use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input, 64)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str, steps: u32) -> Result<u64> {
    let garden = day_21::parse_garden(input)?;

    let result = day_21::random_walk_posibilities(&garden, steps);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let expected = 16;
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        let result = process(input, 6)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
