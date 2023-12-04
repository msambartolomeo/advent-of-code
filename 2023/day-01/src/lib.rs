use std::ops::{Deref, DerefMut};

use anyhow::Result;

#[derive(Debug)]
pub struct CalibrationDocument(Vec<CalibrationValue>);

#[derive(Debug)]
pub struct CalibrationValue {
    pub number: String,
    pub all: String,
}

impl Deref for CalibrationDocument {
    type Target = Vec<CalibrationValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CalibrationDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn parse_calibration_document(input: &str) -> Result<CalibrationDocument> {
    let documents = input
        .lines()
        .map(parse_calibration_value)
        .collect::<Result<Vec<CalibrationValue>>>()?;

    Ok(CalibrationDocument(documents))
}

fn parse_calibration_value(input: &str) -> Result<CalibrationValue> {
    let mut number = String::new();

    input.split_inclusive(|c: char| c.is_ascii_digit());

    for char in input.chars() {
        if let '0'..='9' = char {
            number.push(char);
        }
    }

    Ok(CalibrationValue {
        number,
        all: input.to_owned(),
    })
}
