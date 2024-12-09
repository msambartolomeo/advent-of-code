pub mod parser;

#[derive(Debug, Clone, Copy)]
pub struct DiskItem {
    kind: BlockKind,
    len: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockKind {
    Empty,
    File(u64),
}

impl DiskItem {
    #[must_use]
    pub const fn is_block(&self) -> bool {
        matches!(self.kind, BlockKind::File(_))
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self.kind, BlockKind::Empty)
    }

    #[must_use]
    pub const fn id(&self) -> u64 {
        match self.kind {
            BlockKind::Empty => 0,
            BlockKind::File(id) => id,
        }
    }

    pub fn fragments(&self) -> impl Iterator<Item = BlockKind> {
        std::iter::repeat(self.kind).take(self.len)
    }
}

impl BlockKind {
    #[must_use]
    pub const fn id(&self) -> u64 {
        match self {
            Self::Empty => 0,
            Self::File(id) => *id,
        }
    }
}
