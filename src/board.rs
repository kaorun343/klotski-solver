use crate::{block::Block, bounding_rectangle::BoundingRectangle, direction::ALL_DIRECTIONS};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board<'a> {
    size: [i64; 2],
    blocks: Vec<Block<'a>>,
}

impl<'a> Into<BoundingRectangle> for &'a Board<'a> {
    fn into(self) -> BoundingRectangle {
        BoundingRectangle::new(&[0, 0], &self.size)
    }
}

impl<'a> Board<'a> {
    pub fn new(size: [i64; 2], blocks: Vec<Block<'a>>) -> Self {
        Board { size, blocks }
    }

    pub fn generate_next_states(&self) -> Vec<Self> {
        let blocks = &self.blocks;
        blocks
            .iter()
            .enumerate()
            .flat_map(|(block_idx, block)| {
                ALL_DIRECTIONS
                    .iter()
                    .cloned()
                    .map(move |direction| (block_idx, block.move_to(&direction)))
            })
            .filter(|(_, moved_block)| {
                let board_rectangle: BoundingRectangle = self.into();
                let moved_block_rectangle: BoundingRectangle = moved_block.into();
                moved_block_rectangle.is_inside_of(&board_rectangle)
            })
            .filter(|(block_idx, moved_block)| {
                let moved_block_rectangle: BoundingRectangle = moved_block.into();
                blocks
                    .iter()
                    .enumerate()
                    .filter(|&(other_block_idx, _)| other_block_idx != *block_idx)
                    .all(|(_idx, other_block)| {
                        moved_block_rectangle.is_far_from(&other_block.into())
                    })
            })
            .map(move |(block_idx, moved_block)| {
                let mut blocks = blocks.clone();
                blocks[block_idx] = moved_block.clone();
                Self::new(self.size.clone(), blocks)
            })
            .collect()
    }

    pub fn is_finished(&self) -> bool {
        self.blocks.iter().all(Block::is_at_goal)
    }
}
