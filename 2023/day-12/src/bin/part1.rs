use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let records = day_12::parse_spring_records(input)?;

    let result = records
        .into_par_iter()
        .map(day_12::unknown_spring_posibilities)
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[test]
    fn test_example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = process(input).unwrap();

        assert_eq!(21, result);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_each(#[case] input: &str, #[case] expected: u64) -> Result<()> {
        let record = day_12::parse_record(input)?;

        let result = day_12::unknown_spring_posibilities(record);

        assert_eq!(expected, result);

        Ok(())
    }
}
