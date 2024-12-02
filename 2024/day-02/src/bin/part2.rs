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
        .filter_ok(|v| day_02::is_safe(v, 1))
        .process_results(|it| it.count())? as u64;

    Ok(result)
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
        let expected: u64 = 4;

        let result = process(INPUT.trim())?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_extra() -> Result<()> {
        let input: &str = "
            4 2 3 4 5
            2 3 4 3 5
            1 2 3 4 5
            1 1 3 4 5
            1 3 2 4 5
            10 2 3 4 5
            1 10 3 4 5
            1 2 10 4 5
            1 5 4 3 2
            1 2 5 4 5
            1 3 3 4 5
            1 2 3 4 5
            56 53 55 56 58 60
            56 53 55 50 48 45
        "
        .trim();
        let expected = input.lines().count() as u64;

        let result = process(input)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
