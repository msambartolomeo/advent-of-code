use std::ops::{Deref, DerefMut};

use anyhow::{bail, Ok, Result};

#[derive(Debug)]
pub struct CalibrationDocument(Vec<CalibrationValue>);

#[derive(Debug)]
pub struct CalibrationValue {
    pub number: u32,
    pub word: String,
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
    let mut word = String::new();

    input.split_inclusive(|c: char| c.is_ascii_digit());

    for char in input.chars() {
        match char {
            '0'..='9' => number.push(char),
            'a'..='z' => word.push(char),
            _ => bail!("Invalid in input string character"),
        }
    }

    let first = number.chars().next().unwrap().to_digit(10).unwrap();
    let last = number.chars().last().unwrap().to_digit(10).unwrap();

    let number = first * 10 + last;

    Ok(CalibrationValue { number, word })
}
