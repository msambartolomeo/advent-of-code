use anyhow::Result;
use day_13::{parse_environment, MirrorAccessor};
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
        .filter_map(|m| find_mirror(&m.rows()).or_else(|| find_mirror(&m.columns())))
        .fold(0, |sum, idx| match idx {
            Direction::Vertical(idx) => sum + idx,
            Direction::Horizontal(idx) => sum + 100 * idx,
        });

    Ok(result)
}

fn find_mirror(mirror: &MirrorAccessor) -> Option<Direction> {
    mirror
        .lines()
        .enumerate()
        .tuple_windows()
        .filter_map(|((idx1, v1), (_, v2))| (v1 == v2).then_some(idx1 + 1))
        .find_map(|idx| {
            (0..idx - 1)
                .rev()
                .zip(idx + 1..mirror.len())
                .all(|(id1, id2)| {
                    mirror
                        .nth_line(id1)
                        .zip(mirror.nth_line(id2))
                        .all(|(e1, e2)| e1 == e2)
                })
                .then_some(match mirror {
                    MirrorAccessor::Rows(_) => Direction::Horizontal(idx),
                    MirrorAccessor::Columns(_) => Direction::Vertical(idx),
                })
        })
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
