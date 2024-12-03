use anyhow::Result;
use day_03::Program;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let instructions = day_03::parser::parse(input)?;

    let mut program = Program::from(instructions);
    program.conditionals = false;

    let result = program.run();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 161;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
