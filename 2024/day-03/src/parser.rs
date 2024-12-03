use anyhow::Result;
use regex::Regex;

use crate::Instruction;

pub fn parse(input: &str) -> Result<Vec<Instruction>> {
    Regex::new(r"don't\(\)()()|do\(\)()()|mul\((\d{1,3}),(\d{1,3})\)")
        .expect("Should be valid regex")
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(s, [n1, n2])| match s {
            "don't()" => Ok(Instruction::Dont),
            "do()" => Ok(Instruction::Do),
            _ => Ok(Instruction::Mul(n1.parse()?, n2.parse()?)),
        })
        .collect()
}
