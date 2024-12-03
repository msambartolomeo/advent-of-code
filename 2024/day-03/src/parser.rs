use anyhow::Result;
use regex::Regex;

pub fn parse(input: &str) -> impl Iterator<Item = Result<(u64, u64)>> + '_ {
    let regex = Box::leak(Box::new(
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Should be valid regex"),
    ));

    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| Ok((n1.parse()?, n2.parse()?)))
}
