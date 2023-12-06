use anyhow::{ensure, Context, Result};

#[derive(Debug)]
pub struct BoatRace {
    pub allowed_time: u64,
    pub best_distance: u64,
}

impl From<(u64, u64)> for BoatRace {
    fn from((allowed_time, best_distance): (u64, u64)) -> Self {
        BoatRace {
            allowed_time,
            best_distance,
        }
    }
}

impl BoatRace {
    pub fn new(allowed_time: u64, best_distance: u64) -> Self {
        Self {
            allowed_time,
            best_distance,
        }
    }

    pub fn solve_equation(&self) -> Result<u64> {
        let time = self.allowed_time as f64;
        let distance = self.best_distance as f64;

        let (start, end) = quadratic_formula(1f64, -time, distance)?;

        let (start, end) = (start.floor() as u64, end.ceil() as u64);

        Ok(end - 1 - start)
    }
}

/// Calculates the quadratic formula for real numbers
#[inline]
fn quadratic_formula(a: f64, b: f64, c: f64) -> Result<(f64, f64)> {
    // NOTE: a must not be 0
    ensure!(a != 0f64);

    let discriminant = b * b - 4f64 * a * c;

    // NOTE: discriminant must be positive
    ensure!(discriminant >= 0f64);

    let discriminant_sqrt = discriminant.sqrt();

    Ok((
        (-b - discriminant_sqrt) / (2f64 * a),
        (-b + discriminant_sqrt) / (2f64 * a),
    ))
}

pub fn parse_boat_races(input: &str) -> Result<Vec<BoatRace>> {
    let (times, distances) = input
        .split_once('\n')
        .context("Input must have two lines")?;

    let times = parse_line(times, "Time")?;
    let distances = parse_line(distances, "Distance")?;

    times
        .zip(distances)
        .map(|(t, d)| Ok((t?, d?).into()))
        .collect()
}

#[inline]
fn parse_line<'a>(
    input: &'a str,
    expected_tag: &'a str,
) -> Result<impl Iterator<Item = Result<u64>> + 'a> {
    let (tag, values) = input
        .split_once(':')
        .context("each line must have a : separating tag and content")?;

    ensure!(tag == expected_tag);

    Ok(values
        .trim()
        .split_whitespace()
        .map(|n| Ok(n.parse::<u64>()?)))
}
