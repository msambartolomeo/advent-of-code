use anyhow::Result;
use day_14::{Platform, Rock};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut platform = day_14::parse_platform(input)?;

    slide_platform_north(&mut platform);

    let result = platform
        .iter()
        .map(|(c, r)| match r {
            Rock::Rounded => platform.height as u64 - c.y,
            Rock::Cube => 0,
        })
        .sum();

    Ok(result)
}

fn slide_platform_north(platform: &mut Platform) {
    for x in 0..platform.length {
        let mut rock_to_move = None;
        for y in (0..platform.height).rev() {
            match platform.get(&(x, y).into()) {
                Some(Rock::Rounded) => {
                    if rock_to_move.is_none() {
                        rock_to_move = Some(y);
                    }
                }
                Some(Rock::Cube) => rock_to_move = None,
                None => {
                    if let Some(old_rock) = rock_to_move.take() {
                        platform.remove(&(x, old_rock).into());
                        platform.insert((x, y).into(), Rock::Rounded);

                        rock_to_move = Some(old_rock - 1);
                    }
                }
            }
        }
    }
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
