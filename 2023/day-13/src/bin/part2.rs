use anyhow::Result;
use day_13::parse_environment;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

enum Direction {
    Vertical(usize),
    Horizontal(usize),
}

#[inline]
fn process(input: &str) -> Result<usize> {
    let mirrors = parse_environment(input)?;

    let result = mirrors
        .into_iter()
        .filter_map(|m| {
            m.rows()
                .enumerate()
                .tuple_windows()
                .filter_map(|((idx1, v1), (_, v2))| {
                    let difference = differences(v1, v2);
                    (difference <= 1).then_some((idx1 + 1, difference))
                })
                .find_map(|(idx, mut difference)| {
                    if (0..idx - 1).rev().zip(idx + 1..m.rows).all(|(id1, id2)| {
                        match differences(m.nth_row(id1), m.nth_row(id2)) {
                            0 => true,
                            1 if difference == 0 => {
                                difference = 1;
                                true
                            }
                            _ => false,
                        }
                    }) {
                        if difference == 1 {
                            return Some(Direction::Horizontal(idx));
                        }
                    }
                    None
                })
                .or(m
                    .columns()
                    .enumerate()
                    .tuple_windows()
                    .filter_map(|((idx1, v1), (_, v2))| {
                        let difference = differences(&v1, &v2);
                        (difference <= 1).then_some((idx1 + 1, difference))
                    })
                    .find_map(|(idx, mut difference)| {
                        if (0..idx - 1)
                            .rev()
                            .zip(idx + 1..m.columns)
                            .all(|(id1, id2)| {
                                match differences(&m.nth_column(id1), &m.nth_column(id2)) {
                                    0 => true,
                                    1 if difference == 0 => {
                                        difference = 1;
                                        true
                                    }
                                    _ => false,
                                }
                            })
                        {
                            if difference == 1 {
                                return Some(Direction::Vertical(idx));
                            }
                        }
                        None
                    }))
        })
        .fold(0, |sum, idx| match idx {
            Direction::Vertical(idx) => sum + idx,
            Direction::Horizontal(idx) => sum + 100 * idx,
        });

    Ok(result)
}

fn differences<T: Eq>(v1: &[T], v2: &[T]) -> usize {
    v1.iter().zip(v2).filter(|(e1, e2)| e1 != e2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let result = process(input).unwrap();

        assert_eq!(400, result);
    }
}
