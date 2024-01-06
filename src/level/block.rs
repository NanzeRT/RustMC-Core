#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(i32);

impl BlockId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}
