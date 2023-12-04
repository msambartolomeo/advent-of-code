use day_01::TaintedCalibrationValue;

fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    println!("{result}");
}

#[inline]
fn process(input: &str) -> u32 {
    let document = day_01::parse_calibration_document(input);

    document.iter().map(|s| get_number_value(s)).sum()
}

#[must_use]
fn get_number_value(tainted_value: &TaintedCalibrationValue) -> u32 {
    let mut numbers = vec![];

    for char in tainted_value.chars() {
        let n = char.to_digit(10);

        if let Some(n) = n {
            numbers.push(n);
        }
    }

    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = process(input);

        assert_eq!(142, result);
    }
}
