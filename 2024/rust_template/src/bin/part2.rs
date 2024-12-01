use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(_input: &str) -> Result<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = todo!();

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
