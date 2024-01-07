use crate::City;

use anyhow::{Context, Result};

impl TryFrom<Vec<Vec<u8>>> for City {
    type Error = anyhow::Error;

    fn try_from(matrix: Vec<Vec<u8>>) -> Result<Self> {
        let height = matrix.len();
        let length = matrix.first().context("Must not be empty")?.len();

        Ok(Self {
            length,
            height,
            matrix,
        })
    }
}

/// Parses the city heat map
/// # Errors
/// If any field is not a digit
pub fn city(input: &str) -> Result<City> {
    let matrix = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Ok(u8::try_from(c.to_digit(10).context("Must be a digit")?)?))
                .collect::<Result<Vec<u8>>>()
        })
        .collect::<Result<Vec<Vec<u8>>>>()?;

    matrix.try_into()
}
