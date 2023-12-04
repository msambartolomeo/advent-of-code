use day_01::TaintedCalibrationValue;

fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    println!("{result}");
}

#[inline]
fn process(input: &str) -> u32 {
    let document = day_01::parse_calibration_document(input);

    document.iter().map(|s| get_real_value(s)).sum()
}

#[must_use]
fn get_real_value(tainted_value: &TaintedCalibrationValue) -> u32 {
    let mut it = tainted_value.numbers();

    let first = it.next().expect("at least one number in the input");
    let last = match it.last() {
        Some(n) => n,
        None => first,
    };

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = process(input);

        assert_eq!(281, result);
    }
}
