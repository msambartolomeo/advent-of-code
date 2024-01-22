use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input, 26_501_365)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str, steps: u32) -> Result<u64> {
    let mut garden = day_21::parse_garden(input)?;

    garden.make_infinite();

    let result = day_21::random_walk_posibilities(&garden, steps);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1_594)]
    #[case(100, 6_536)]
    #[case(500, 167_004)]
    #[case(1_000, 668_697)]
    #[case(5_000, 16_733_044)]
    fn test_example(#[case] steps: u32, #[case] expected: u64) -> Result<()> {
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

        let result = process(input, steps)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
