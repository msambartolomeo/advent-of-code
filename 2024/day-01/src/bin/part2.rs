use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let _input = day_01::parser::parse(input)?;

    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = todo!();
        let input = todo!();

        let result = process(input)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
