#[must_use]
#[inline]
pub fn holiday_ascii_string_helper(string: &str) -> u8 {
    string.as_bytes().iter().fold(0, |current_value, &c| {
        current_value.wrapping_add(c).wrapping_mul(17)
    })
}

#[inline]
pub fn parse_manual(input: &str) -> impl Iterator<Item = &str> {
    input.lines().flat_map(|l| l.split(','))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash_algorithm(#[case] string: &str, #[case] hash: u8) {
        assert_eq!(hash, holiday_ascii_string_helper(string))
    }
}
