use std::ops::{Deref, DerefMut};

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

#[must_use]
pub fn parse_calibration_document(input: &str) -> CalibrationDocument {
    let documents = input.lines().map(ToOwned::to_owned).collect();

    CalibrationDocument(documents)
}

pub struct NumberIterator<'a>(&'a str);

const NUMBER_TRANSLATOR: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl<'a> Iterator for NumberIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        for (num, n) in NUMBER_TRANSLATOR {
            if self.0.starts_with(num) {
                // NOTE: Use len - 1 because of oneight, it should be counted as 18
                self.0 = &self.0[num.len() - 1..];
                return Some(n);
            }
        }

        if let Some(n) = self.0.chars().next()?.to_digit(10) {
            self.0 = &self.0[1..];
            return Some(n);
        }

        if self.0.is_empty() {
            self.0 = &self.0[1..];
            return self.next();
        }

        None
    }
}

impl<'a> From<&'a str> for NumberIterator<'a> {
    fn from(value: &'a str) -> Self {
        NumberIterator(value)
    }
}
