use anyhow::Result;
use day_04::U;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let word_search = day_04::parser::parse(input)?;

    let U(x, y) = word_search.size();

    let mut result = 0;

    for i in 0..x {
        for j in 0..y {
            result += word_search.search_xmas(U(i, j));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 18;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
