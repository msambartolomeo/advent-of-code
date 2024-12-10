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
                day_10::trails_dfs(&topographic_map, idx)
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
        1111201
        1143211
        1151121
        1165431
        1171141
        1187651
        1191111
    ";

    const INPUT2: &str = "
        1190119
        1111198
        1112117
        6543456
        7651987
        8761111
        9871111
    ";

    const INPUT3: &str = "
        012345
        123456
        234567
        345678
        416789
        567891
    ";

    const INPUT4: &str = "
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
    #[case(INPUT1, 3)]
    #[case(INPUT2, 13)]
    #[case(INPUT3, 227)]
    #[case(INPUT4, 81)]
    fn test_example(#[case] input: &str, #[case] expected: u64) -> Result<()> {
        let result = process(input)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
