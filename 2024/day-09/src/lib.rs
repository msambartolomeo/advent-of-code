pub mod parser;

#[derive(Debug, Clone, Copy)]
pub enum DiskItem {
    EmptySpace { current: usize, len: u64 },
    Block { id: u64, current: usize, len: u64 },
}

impl DiskItem {
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block { .. })
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::EmptySpace { .. })
    }

    pub fn id(&self) -> u64 {
        match self {
            DiskItem::EmptySpace { .. } => 0,
            DiskItem::Block { id, .. } => *id,
        }
    }
}
