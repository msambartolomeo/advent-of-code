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
    let (hl1, hl2) = day_01::parser::parse(input)?;

    let counts1 = hl1.into_iter().map(|n| n.0).counts();
    let counts2 = hl2.into_iter().map(|n| n.0).counts();

    let result = counts1
        .into_iter()
        .map(|(id, count)| {
            let appearences = counts2.get(&id).copied().unwrap_or_default();

            id * appearences as u64 * count as u64
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 31;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
