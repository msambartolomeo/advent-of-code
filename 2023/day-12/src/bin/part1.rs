use anyhow::Result;
use day_12::{Record, Spring};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let records = day_12::parse_spring_records(input);

    let result = records.process_results(|it| it.map(|r| unknown_spring_posibilities(&r)).sum())?;

    Ok(result)
}

pub fn unknown_spring_posibilities(record: &Record) -> u32 {
    unknown_spring_posibilities_rec(&record.0, &record.1, 0)
}

fn unknown_spring_posibilities_rec(
    springs: &[Spring],
    damaged_groups: &[u32],
    damaged_count: u32,
) -> u32 {
    match springs.split_first() {
        Some((Spring::Operational, springs)) => {
            if damaged_groups.first() == Some(&damaged_count) {
                unknown_spring_posibilities_rec(springs, &damaged_groups[1..], 0)
            } else if damaged_count == 0 {
                unknown_spring_posibilities_rec(springs, damaged_groups, 0)
            } else {
                0
            }
        }
        Some((Spring::Damaged, springs)) => match damaged_groups.first() {
            None => 0,
            Some(&group) if group == damaged_count => 0,
            _ => unknown_spring_posibilities_rec(springs, damaged_groups, damaged_count + 1),
        },
        Some((Spring::Unknown, _)) => {
            // Divide into damaged and operational posibilities
            let mut springs = springs.to_vec();

            springs[0] = Spring::Damaged;
            let damaged_posibilities =
                unknown_spring_posibilities_rec(&springs, damaged_groups, damaged_count);

            springs[0] = Spring::Operational;

            let operational_posibilities =
                unknown_spring_posibilities_rec(&springs, damaged_groups, damaged_count);

            damaged_posibilities + operational_posibilities
        }
        None => u32::from(
            damaged_groups.is_empty()
                || (damaged_groups.len() == 1 && damaged_groups[0] == damaged_count),
        ),
    }
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
    fn test_each(#[case] input: &str, #[case] expected: u32) -> Result<()> {
        let record = day_12::parse_record(input)?;

        let result = unknown_spring_posibilities(record);

        assert_eq!(expected, result);

        Ok(())
    }
}
