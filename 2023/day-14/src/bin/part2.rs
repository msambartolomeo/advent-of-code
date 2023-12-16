use std::collections::HashSet;

use anyhow::Result;
use day_14::{Platform, Rock};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

const CYCLES: u64 = 1_000_000_000;

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut platform = day_14::parse_platform(input)?;

    let start_to_loop_size = slide_until_loop(&mut platform);

    let loop_size = slide_until_loop(&mut platform);

    let missing_cycles = (CYCLES - start_to_loop_size) % loop_size;

    for _ in 0..missing_cycles {
        day_14::slide_platform_cycle(&mut platform);
    }

    let result = platform
        .iter()
        .map(|(c, r)| match r {
            Rock::Rounded => platform.height as u64 - c.y,
            Rock::Cube => 0,
        })
        .sum();

    Ok(result)
}

fn slide_until_loop(platform: &mut Platform) -> u64 {
    let mut past_platforms = HashSet::new();

    past_platforms.insert(platform.to_string());

    let mut idx = 1;

    loop {
        day_14::slide_platform_cycle(platform);

        let string = platform.to_string();

        if past_platforms.contains(&string) {
            break idx;
        }

        past_platforms.insert(string);

        idx += 1;
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

        assert_eq!(64, result);
    }

    #[test]
    fn test_north() {
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

        let mut platform = day_14::parse_platform(input).unwrap();

        day_14::slide_platform_north(&mut platform);

        let result = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
";

        assert_eq!(result, platform.to_string());
    }

    #[test]
    fn test_west() {
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

        let mut platform = day_14::parse_platform(input).unwrap();

        day_14::slide_platform_west(&mut platform);

        let result = "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....
";

        assert_eq!(result, platform.to_string());
    }

    #[test]
    fn test_south() {
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

        let mut platform = day_14::parse_platform(input).unwrap();

        day_14::slide_platform_south(&mut platform);

        let result = ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O
";

        assert_eq!(result, platform.to_string());
    }

    #[test]
    fn test_east() {
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

        let mut platform = day_14::parse_platform(input).unwrap();

        day_14::slide_platform_east(&mut platform);

        let result = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....
";

        assert_eq!(result, platform.to_string());
    }

    #[test]
    fn test_cycles() {
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

        let mut platform = day_14::parse_platform(input).unwrap();

        day_14::slide_platform_cycle(&mut platform);

        let first = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";

        assert_eq!(first, platform.to_string());

        day_14::slide_platform_cycle(&mut platform);

        let second = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";

        assert_eq!(second, platform.to_string());

        day_14::slide_platform_cycle(&mut platform);

        let third = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
";

        assert_eq!(third, platform.to_string());
    }
}
