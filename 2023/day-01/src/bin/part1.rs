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

    Ok(document.iter().map(|s| day_01::get_number_value(&s)).sum())
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
