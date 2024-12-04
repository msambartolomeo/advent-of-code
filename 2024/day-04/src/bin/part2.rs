use anyhow::Result;
use day_04::{Letter, I, U};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

const MAS_IN_X_SEARCH_OPTIONS: [[I; 4]; 4] = [
    [I(-1, -1), I(1, 1), I(1, -1), I(-1, 1)],
    [I(-1, -1), I(1, 1), I(-1, 1), I(1, -1)],
    [I(1, 1), I(-1, -1), I(1, -1), I(-1, 1)],
    [I(1, 1), I(-1, -1), I(-1, 1), I(1, -1)],
];

const AMSMS: [Letter; 5] = [Letter::A, Letter::M, Letter::S, Letter::M, Letter::S];

#[inline]
fn process(input: &str) -> Result<u64> {
    let word_search = day_04::parser::parse(input)?;

    let U(x, y) = word_search.size();

    let mut result = 0;

    for i in 0..x {
        for j in 0..y {
            result += word_search.search(U(i, j), &AMSMS, &MAS_IN_X_SEARCH_OPTIONS);
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
        let expected: u64 = 9;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
