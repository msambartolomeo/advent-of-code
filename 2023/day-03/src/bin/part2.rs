use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let schematic = day_03::parse_schematic(input)?;

    let result = schematic
        .symbols
        .iter()
        .flat_map(|p| p.gear_ratio(&schematic.parts))
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

        assert_eq!(467835, result);
    }
}
