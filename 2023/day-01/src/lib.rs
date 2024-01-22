use std::{fmt::Display, ops::Deref};

#[derive(Debug)]
pub struct CalibrationDocument<'a>(Vec<TaintedCalibrationValue<'a>>);

impl<'a> Deref for CalibrationDocument<'a> {
    type Target = Vec<TaintedCalibrationValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TaintedCalibrationValue<'a>(&'a str);

impl<'a> TaintedCalibrationValue<'a> {
    #[must_use]
    pub fn numbers(&self) -> NumberIterator<'a> {
        NumberIterator(self)
    }
}

impl<'a> Deref for TaintedCalibrationValue<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Display for TaintedCalibrationValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> IntoIterator for TaintedCalibrationValue<'a> {
    type Item = u32;

    type IntoIter = NumberIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.numbers()
    }
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

        let next = self.0.chars().next()?;
        self.0 = &self.0[1..];
        next.to_digit(10).map_or_else(|| self.next(), Some)
    }
}

#[must_use]
pub fn parse_calibration_document(input: &str) -> CalibrationDocument {
    let documents = input.lines().map(TaintedCalibrationValue).collect();

    CalibrationDocument(documents)
}
