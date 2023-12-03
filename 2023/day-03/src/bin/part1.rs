use anyhow::{Ok, Result};
use day_03::parse_schematic;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let schematic = parse_schematic(input)?;

    let result = schematic
        .parts
        .iter()
        .filter(|p| p.is_valid(&schematic.symbols))
        .map(|p| p.id)
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process(input).expect("Process failure");

        assert_eq!(4361, result);
    }
}
