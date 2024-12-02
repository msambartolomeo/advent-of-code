use std::cmp::Ordering;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let reports = day_02::parser::parse(input);

    let result = reports
        .filter_ok(|v| is_safe(v))
        .process_results(|it| it.count())? as u64;

    Ok(result)
}

fn is_safe(v: &[u64]) -> bool {
    let first = v[0];

    v.iter()
        .skip(1)
        .try_fold((first, Ordering::Equal), |(last, ordering), &next| {
            let new_ord = next.cmp(&last);
            let diff = last.abs_diff(next);

            if diff == 0 || diff > 3 {
                Err(())
            } else if matches!(ordering, Ordering::Equal) || new_ord == ordering {
                Ok((next, new_ord))
            } else {
                Err(())
            }
        })
        .is_ok()
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 2;

        let result = process(INPUT.trim())?;

        assert_eq!(expected, result);

        Ok(())
    }
}
