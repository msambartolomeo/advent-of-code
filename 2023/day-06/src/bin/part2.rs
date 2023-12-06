use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let long_race = day_06::parse_long_boat_race(input)?;

    long_race.solve_equation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30\nDistance:  9  41  200";

        let result = process(input).unwrap();

        assert_eq!(71503, result);
    }
}
