use anyhow::{ensure, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct BoatRace {
    pub allowed_time: u32,
    pub best_distance: u32,
}

impl From<(u32, u32)> for BoatRace {
    fn from((allowed_time, best_distance): (u32, u32)) -> Self {
        Self {
            allowed_time,
            best_distance,
        }
    }
}

impl BoatRace {
    #[must_use]
    pub const fn new(allowed_time: u32, best_distance: u32) -> Self {
        Self {
            allowed_time,
            best_distance,
        }
    }

    /// # Errors
    ///
    /// Returns error if the equation cannot be solved for real numbers
    pub fn solve_equation(&self) -> Result<u64> {
        let time = f64::from(self.allowed_time);
        let distance = f64::from(self.best_distance);

        let (start, end) = quadratic_formula(1f64, -time, distance)?;

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let (start, end) = (start.floor() as u64, end.ceil() as u64);

        Ok(end - 1 - start)
    }
}

/// Calculates the quadratic formula for real numbers
#[inline]
fn quadratic_formula(a: f64, b: f64, c: f64) -> Result<(f64, f64)> {
    // NOTE: a must not be 0
    ensure!(a > f64::EPSILON);

    let discriminant = b.mul_add(b, -4f64 * a * c);

    // NOTE: discriminant must be positive
    ensure!(discriminant >= 0f64);

    let discriminant_sqrt = discriminant.sqrt();

    Ok((
        (-b - discriminant_sqrt) / (2f64 * a),
        (-b + discriminant_sqrt) / (2f64 * a),
    ))
}

/// Parses the boat races
///
/// # Errors
///
/// Errors if the input is invalid
pub fn parse_boat_races(input: &str) -> Result<Vec<BoatRace>> {
    let (times, distances) = parse_times_and_distances(input)?;

    times
        .zip(distances)
        .map(|(t, d)| Ok((t?, d?).into()))
        .collect()
}

/// Parses the single long race
///
/// # Errors
///
/// Errors if the input is invalid
pub fn parse_long_boat_race(input: &str) -> Result<BoatRace> {
    let (times, distances) = parse_times_and_distances(input)?;

    let time = times.process_results(|mut it| it.join(""))?.parse()?;
    let distance = distances.process_results(|mut it| it.join(""))?.parse()?;

    Ok((time, distance).into())
}

#[inline]
fn parse_times_and_distances(
    input: &str,
) -> Result<(
    impl Iterator<Item = Result<u32>> + '_,
    impl Iterator<Item = Result<u32>> + '_,
)> {
    let (times, distances) = input
        .split_once('\n')
        .context("Input must have two lines")?;

    let times = parse_line(times, "Time")?;
    let distances = parse_line(distances, "Distance")?;

    Ok((times, distances))
}

#[inline]
fn parse_line<'a>(
    input: &'a str,
    expected_tag: &str,
) -> Result<impl Iterator<Item = Result<u32>> + 'a> {
    let (tag, values) = input
        .split_once(':')
        .context("each line must have a : separating tag and content")?;

    ensure!(tag == expected_tag);

    Ok(values.split_whitespace().map(|n| Ok(n.parse()?)))
}
