use anyhow::Result;

pub fn parse_oasis_report(input: &str) -> impl Iterator<Item = Result<Vec<i32>>> + '_ {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| Ok(n.parse()?)).collect())
}
