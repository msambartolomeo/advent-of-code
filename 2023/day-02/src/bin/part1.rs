use anyhow::Result;
use day_02::{Bag, Game};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let games = day_02::parse_games(input)?;

    let result = process(&games);

    println!("{result}");

    Ok(())
}

fn process(games: &[Game]) -> u32 {
    let bag = Bag::new(12, 13, 14);

    games
        .into_iter()
        .filter(|g| g.iter().all(|b| b.is_contained(&bag)))
        .map(|g| g.get_id())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = day_02::parse_games(input).expect("Invalid input");

        let result = process(&games);

        assert_eq!(8, result);
    }
}
