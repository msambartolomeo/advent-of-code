use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let races = day_06::parse_boat_races(input)?;

    races.into_iter().map(|r| r.solve_equation()).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    use day_06::*;

    use rstest::*;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30\nDistance:  9  41  200";

        let result = process(input).unwrap();

        assert_eq!(288, result);
    }

    #[rstest]
    #[case(BoatRace::new(7, 9), 4)]
    #[case(BoatRace::new(15, 40), 8)]
    #[case(BoatRace::new(30, 200), 9)]
    fn test_race(#[case] race: BoatRace, #[case] expected: u32) {
        let result = race.solve_equation().unwrap();

        assert_eq!(expected, result);
    }
}
