use crate::block::Block;

#[derive(Debug)]
pub struct Board {
    size: [u64; 2],
    blocks: Vec<Block>,
}

impl Board {
    pub fn new(size: [u64; 2], blocks: Vec<Block>) -> Self {
        Board { size, blocks }
    }
}
