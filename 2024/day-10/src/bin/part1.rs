use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let topographic_map = day_10::parser::parse(input)?;

    let result = topographic_map
        .iter()
        .map(|(idx, &n)| {
            if n == 0 {
                day_10::trail_tails_dfs(&topographic_map, idx)
            } else {
                0
            }
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    use rstest::rstest;

    const INPUT1: &str = "
        0123
        1234
        8765
        9876
    ";

    const INPUT2: &str = "
        1110111
        1111111
        1112111
        6543456
        7111117
        8111118
        9111119
    ";

    const INPUT3: &str = "
        1190119
        1111198
        1112117
        6543456
        7651987
        8761111
        9871111
    ";

    const INPUT4: &str = "
        1011911
        2111811
        3111711
        4567654
        1118113
        1119112
        1111101
    ";

    const INPUT5: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    #[rstest]
    #[case(INPUT1, 1)]
    #[case(INPUT2, 2)]
    #[case(INPUT3, 4)]
    #[case(INPUT4, 3)]
    #[case(INPUT5, 36)]
    fn test_example(#[case] input: &str, #[case] expected: u64) -> Result<()> {
        let result = process(input)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
