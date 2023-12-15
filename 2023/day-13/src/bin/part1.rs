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
                .filter_map(|((idx1, v1), (_, v2))| (v1 == v2).then_some(idx1 + 1))
                .find_map(|idx| {
                    (0..idx)
                        .rev()
                        .zip(idx..m.rows)
                        .all(|(id1, id2)| m.nth_row(id1) == m.nth_row(id2))
                        .then(|| Direction::Horizontal(idx))
                })
                .or(m
                    .columns()
                    .enumerate()
                    .tuple_windows()
                    .filter_map(|((idx1, v1), (_, v2))| (v1 == v2).then_some(idx1 + 1))
                    .find_map(|idx| {
                        (0..idx)
                            .rev()
                            .zip(idx..m.columns)
                            .all(|(id1, id2)| m.nth_column(id1) == m.nth_column(id2))
                            .then(|| Direction::Vertical(idx))
                    }))
        })
        .fold(0, |sum, idx| match idx {
            Direction::Vertical(idx) => sum + idx,
            Direction::Horizontal(idx) => sum + 100 * idx,
        });

    Ok(result)
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

        assert_eq!(405, result);
    }
}
