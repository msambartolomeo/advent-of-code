use std::collections::HashMap;

use anyhow::{format_err, Result};
use winnow::ascii::{alpha1, digit1};
use winnow::combinator::{alt, separated_pair, terminated};
use winnow::{PResult, Parser};

pub type Label = String;
pub type HASH = u8;
pub type HASHMAP = HashMap<HASH, Vec<Lens>>;

#[derive(Debug)]
pub struct Lens {
    pub label: Label,
    pub focal_length: u8,
}

pub enum InitializationOperation {
    Add(Lens),
    Remove(Label),
}

/// Parses the instruction returning the operation
///
/// # Errors
/// If the instruction is not a label followed by '-' or '=' and then a number
pub fn parse_instruction(input: &str) -> Result<InitializationOperation> {
    alt((add_instruction, remove_instruction))
        .parse(input)
        .map_err(|e| format_err!(e.to_string()))
}

fn add_instruction(input: &mut &str) -> PResult<InitializationOperation> {
    separated_pair(alpha1.parse_to(), '=', digit1.parse_to())
        .map(|(label, focal_length)| Lens {
            label,
            focal_length,
        })
        .map(InitializationOperation::Add)
        .parse_next(input)
}

fn remove_instruction(input: &mut &str) -> PResult<InitializationOperation> {
    terminated(alpha1.parse_to(), '-')
        .map(InitializationOperation::Remove)
        .parse_next(input)
}

#[must_use]
pub fn holiday_ascii_string_helper_manual_arrangement_procedure() -> HASHMAP {
    HashMap::new()
}

#[must_use]
#[inline]
pub fn holiday_ascii_string_helper(string: &str) -> HASH {
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
    fn test_hash_algorithm(#[case] string: &str, #[case] hash: HASH) {
        assert_eq!(hash, holiday_ascii_string_helper(string))
    }
}
