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
        .map(|s| {
            println!("{s}");
            let n = day_01::get_real_value(&s);
            println!("{n}");
            n
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = process(input).expect("Process failure");

        assert_eq!(281, result);
    }
}
