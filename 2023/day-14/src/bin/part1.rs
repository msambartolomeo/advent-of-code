use anyhow::Result;
use day_14::Rock;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<usize> {
    let mut platform = day_14::parse_platform(input)?;

    slide_platform_north(&mut platform);

    let result = platform
        .into_iter()
        .map(|column| {
            column
                .into_iter()
                .rev()
                .enumerate()
                .map(|(i, r)| match r {
                    Rock::Rounded => i + 1,
                    Rock::Cube | Rock::Empty => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(result)
}

fn slide_platform_north(platform: &mut Vec<Vec<Rock>>) {
    for column in platform.iter_mut() {
        let mut swaps = Vec::new();
        let mut rock_to_move = None;
        for (current, rock) in column.iter().enumerate().rev() {
            match rock {
                Rock::Rounded => {
                    if rock_to_move.is_none() {
                        rock_to_move = Some(current);
                    }
                }
                Rock::Cube => rock_to_move = None,
                Rock::Empty => {
                    if let Some(other) = rock_to_move.take() {
                        swaps.push((other, current));
                        rock_to_move = Some(other - 1);
                    }
                }
            }
        }

        swaps.into_iter().for_each(|(i1, i2)| {
            let rock = std::mem::take(&mut column[i1]);
            column[i2] = rock;
        })
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
