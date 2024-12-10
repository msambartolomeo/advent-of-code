use anyhow::{Context, Result};

use crate::Matrix;

pub fn parse(input: &str) -> Result<Matrix<u64>> {
    let vec = input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| Ok(c.to_digit(10).context("Invalid digit")?.into()))
                .collect()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Matrix::from(vec))
}
