use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let modules = day_20::parse_module_configuration(input)?;

    dbg!(modules);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invariable() -> Result<()> {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let result = process(input)?;

        assert_eq!(32000000, result);

        Ok(())
    }

    #[test]
    fn test_variable() -> Result<()> {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        let result = process(input)?;

        assert_eq!(11687500, result);

        Ok(())
    }
}
