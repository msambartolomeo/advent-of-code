use anyhow::{Context, Result};

use day_01::TaintedCalibrationValue;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let document = day_01::parse_calibration_document(input);

    let result = document
        .iter()
        .map(|s| get_real_value(s).context(format!("Line {s} does not contains numbers")))
        .sum::<Result<u32>>()?;

    Ok(result)
}

#[must_use]
fn get_real_value(tainted_value: &TaintedCalibrationValue) -> Option<u32> {
    let mut it = tainted_value.into_iter();

    let first = it.next()?;
    let last = it.last().map_or(first, |n| n);

    Some(first * 10 + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = process(input)?;

        assert_eq!(281, result);

        Ok(())
    }
}
