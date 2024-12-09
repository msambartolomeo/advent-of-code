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

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 1928;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
