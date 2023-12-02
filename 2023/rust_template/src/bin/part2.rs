use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {}
}
