use crate::level::block::BlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStateId {
    pub(crate) id: i32,
    pub(crate) block_id: BlockId,
}

impl BlockStateId {
    pub fn new(id: i32, block_id: i32) -> Self {
        Self {
            id,
            block_id: BlockId::new(block_id),
        }
    }

    pub fn is_block(&self, block_id: BlockId) -> bool {
        self.block_id == block_id
    }
}
