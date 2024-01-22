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
        .map(|s| get_number_value(s).context(format!("Line {s} does not have two numbers")))
        .sum::<Result<u32>>()?;

    Ok(result)
}

#[must_use]
fn get_number_value(tainted_value: &TaintedCalibrationValue) -> Option<u32> {
    let numbers = tainted_value
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let first = numbers.first()?;
    let last = numbers.last()?;

    Some(first * 10 + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = process(input)?;

        assert_eq!(142, result);

        Ok(())
    }
}
