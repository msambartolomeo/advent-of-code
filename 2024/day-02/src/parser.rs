use anyhow::Result;

pub fn parse(input: &str) -> impl Iterator<Item = Result<Vec<u64>>> + '_ {
    input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|level| Ok(level.parse()?))
            .collect()
    })
}
