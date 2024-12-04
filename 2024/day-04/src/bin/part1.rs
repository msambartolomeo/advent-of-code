use anyhow::Result;
use day_04::{Letter, I, U};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

const XMAS_SEARCH_OPTIONS: [[I; 3]; 8] = [
    [I(0, 1), I(0, 2), I(0, 3)],
    [I(0, -1), I(0, -2), I(0, -3)],
    [I(1, 0), I(2, 0), I(3, 0)],
    [I(-1, 0), I(-2, 0), I(-3, 0)],
    [I(1, 1), I(2, 2), I(3, 3)],
    [I(-1, -1), I(-2, -2), I(-3, -3)],
    [I(1, -1), I(2, -2), I(3, -3)],
    [I(-1, 1), I(-2, 2), I(-3, 3)],
];

const XMAS: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

#[inline]
fn process(input: &str) -> Result<u64> {
    let word_search = day_04::parser::parse(input)?;

    let U(x, y) = word_search.size();

    let mut result = 0;

    for i in 0..x {
        for j in 0..y {
            result += word_search.search(U(i, j), &XMAS, &XMAS_SEARCH_OPTIONS);
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
