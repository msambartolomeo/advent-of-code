use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let document = day_01::parse_calibration_document(input)?;

    Ok(document
        .iter()
        .map(|d| first_and_last_number(&d.number))
        .sum())
}

fn first_and_last_number(n: &str) -> u32 {
    let first = n.chars().next().unwrap().to_digit(10).unwrap();
    let last = n.chars().last().unwrap().to_digit(10).unwrap();

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = process(input).expect("Process failure");

        assert_eq!(142, result);
    }
}
