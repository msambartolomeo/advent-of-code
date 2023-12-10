use anyhow::Result;

pub fn parse_oasis_report(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| Ok(n.parse()?)).collect())
        .collect()
}
