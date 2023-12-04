use std::ops::{Deref, DerefMut};

use anyhow::Result;

#[derive(Debug)]
pub struct CalibrationDocument(Vec<String>);

impl Deref for CalibrationDocument {
    type Target = Vec<String>;

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
    let documents = input.lines().map(|l| l.to_owned()).collect();

    Ok(CalibrationDocument(documents))
}

pub fn get_number_value(tainted_value: &str) -> u32 {
    let mut number = vec![];

    for char in tainted_value.chars() {
        let n = char.to_digit(10);

        if let Some(n) = n {
            number.push(n);
        }
    }

    let first = number.iter().next().unwrap();
    let last = number.iter().last().unwrap();

    first * 10 + last
}
