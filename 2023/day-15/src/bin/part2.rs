use anyhow::Result;
use day_15::InitializationOperation;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let instructions = day_15::parse_manual(input);

    let mut hashmap = day_15::holiday_ascii_string_helper_manual_arrangement_procedure();

    for instruction in instructions {
        let operation = day_15::parse_instruction(instruction)?;

        match operation {
            InitializationOperation::Add(lens) => {
                let hash = day_15::holiday_ascii_string_helper(&lens.label);

                let lens_box = hashmap.entry(hash).or_default();

                let position = lens_box.iter().position(|l| l.label == lens.label);

                match position {
                    Some(i) => lens_box[i] = lens,
                    None => lens_box.push(lens),
                }
            }
            InitializationOperation::Remove(label) => {
                let hash = day_15::holiday_ascii_string_helper(&label);

                if let Some(lens_box) = hashmap.get_mut(&hash) {
                    lens_box.retain(|l| l.label != label);
                }
            }
        }
    }

    let result = hashmap
        .into_iter()
        .flat_map(|(idx, lens_box)| {
            lens_box.into_iter().enumerate().map(move |(slot, lens)| {
                (u64::from(idx + 1)) * (slot as u64 + 1) * u64::from(lens.focal_length)
            })
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = process(input).unwrap();

        assert_eq!(145, result);
    }
}
