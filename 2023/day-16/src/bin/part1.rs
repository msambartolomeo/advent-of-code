use std::collections::HashSet;
use std::rc::Rc;

use anyhow::Result;
use day_16::{Contraption, Direction, LightDeflector, SplitterOutput};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let contraption = day_16::parse_contraption(input)?;

    let mut contraption = Rc::new(contraption);

    let result = energize_contraption(&mut contraption)
        .collect::<HashSet<_>>()
        .len() as u64;

    Ok(result)
}

#[inline]
#[must_use]
fn energize_contraption(
    contraption: &mut Rc<Contraption>,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    energize_direction(contraption, Direction::Right, Some((0, 0)))
}

fn energize_direction(
    contraption: &mut Rc<Contraption>,
    direction: Direction,
    position: Option<(usize, usize)>,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    match position {
        Some(position) => {
            match Rc::get_mut(contraption).unwrap().get_mut(&position) {
                Some(deflector) => match deflector {
                    LightDeflector::Mirror(mirror) => match mirror.get_output(direction) {
                        Some(new_direction) => energize_next(contraption, new_direction, position),
                        None => Box::new(std::iter::empty()),
                    },
                    LightDeflector::Splitter(splitter) => match splitter.get_output(direction) {
                        Some(SplitterOutput::Split([d1, d2])) => {
                            let p1 = contraption.next_position(position, d1);
                            let p2 = contraption.next_position(position, d2);

                            let it = energize_direction(contraption, d1, p1)
                                .chain(energize_direction(contraption, d2, p2));

                            Box::new(std::iter::once(position).chain(it))
                        }
                        Some(SplitterOutput::Continue) => {
                            energize_next(contraption, direction, position)
                        }
                        None => Box::new(std::iter::empty()),
                    },
                },
                None => energize_next(contraption, direction, position),
            }
        }
        None => Box::new(std::iter::empty()),
    }
}

#[inline]
pub fn energize_next(
    contraption: &mut Rc<Contraption>,
    direction: Direction,
    position: (usize, usize),
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let new_position = contraption.next_position(position, direction);
    let it = energize_direction(contraption, direction, new_position);

    Box::new(std::iter::once(position).chain(it))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        let result = process(input)?;

        assert_eq!(46, result);

        Ok(())
    }
}
