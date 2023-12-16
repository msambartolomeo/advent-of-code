use anyhow::Result;
use day_14::Rock;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut platform = day_14::parse_platform(input)?;

    day_14::slide_platform_north(&mut platform);

    let result = platform
        .iter()
        .map(|(c, r)| match r {
            Rock::Rounded => platform.height as u64 - c.y,
            Rock::Cube => 0,
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let result = process(input).unwrap();

        assert_eq!(136, result);
    }
}
