use anyhow::Result;
use day_06::BoatRace;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
#[allow(unused)]
fn process(input: &str) -> Result<u64> {
    let races = day_06::parse_boat_races(input)?;

    let mut long_race = BoatRace::new(0, 0);

    for race in races {
        append_number(&mut long_race.allowed_time, race.allowed_time);
        append_number(&mut long_race.best_distance, race.best_distance);
    }

    long_race.solve_equation()
}

fn append_number(n: &mut u64, m: u64) {
    let length = (m as f64).log10() as u32 + 1;

    *n *= 10u64.pow(length);

    *n += m;
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30\nDistance:  9  41  200";

        let result = process(input).unwrap();

        assert_eq!(71503, result);
    }
}
