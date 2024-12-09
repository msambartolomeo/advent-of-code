use anyhow::{Context, Result};

use crate::{BlockKind, DiskItem};

pub fn parse(input: &str) -> Result<Vec<DiskItem>> {
    input
        .trim()
        .char_indices()
        .map(|(i, c)| {
            let len = c.to_digit(10).context("Invalid digit")? as usize;

            if i % 2 == 0 {
                let id = i as u64 / 2;
                Ok(DiskItem {
                    kind: BlockKind::File(id),
                    len,
                })
            } else {
                Ok(DiskItem {
                    kind: BlockKind::Empty,
                    len,
                })
            }
        })
        .collect()
}
