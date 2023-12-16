fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    println!("{result}");
}

#[inline]
fn process(input: &str) -> u64 {
    day_15::parse_manual(input)
        .map(|s| u64::from(day_15::holiday_ascii_string_helper(s)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = process(input);

        assert_eq!(1320, result);
    }
}
