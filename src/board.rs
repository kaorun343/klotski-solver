use fnv::FnvHashSet;

use crate::{block::Block, bounding_rectangle::BoundingRectangle, direction::ALL_DIRECTIONS};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board<'a> {
    size: [i64; 2],
    blocks: Vec<Block<'a>>,
}

impl<'a> Board<'a> {
    #[inline]
    pub fn new(size: [i64; 2], blocks: Vec<Block<'a>>) -> Self {
        Board { size, blocks }
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.blocks.iter().all(Block::is_at_goal)
    }

    #[inline]
    pub fn run(
        boards: FnvHashSet<Board<'a>>,
        collection: &FnvHashSet<Board<'a>>,
    ) -> FnvHashSet<Board<'a>> {
        boards
            .iter()
            .flat_map(|board| {
                let board_rectangle = BoundingRectangle::new(&[0, 0], &board.size);
                let blocks = &board.blocks;

                blocks
                    .iter()
                    .enumerate()
                    .flat_map(|(block_idx, block)| {
                        ALL_DIRECTIONS
                            .iter()
                            .cloned()
                            .map(move |direction| (block_idx, block.move_to(&direction)))
                    })
                    .filter(move |(_, moved_block)| {
                        let moved_block_rectangle: BoundingRectangle = moved_block.into();
                        moved_block_rectangle.is_inside_of(&board_rectangle)
                    })
                    .filter(move |(block_idx, moved_block)| {
                        let moved_block_rectangle: BoundingRectangle = moved_block.into();
                        blocks
                            .iter()
                            .enumerate()
                            .filter(|&(other_block_idx, _)| other_block_idx != *block_idx)
                            .all(|(_, other_block)| {
                                moved_block_rectangle.is_far_from(&other_block.into())
                            })
                    })
                    .map(move |(block_idx, moved_block)| {
                        let mut blocks = blocks.clone();
                        blocks[block_idx] = moved_block.clone();
                        Board::new(board.size.clone(), blocks)
                    })
            })
            .filter(|board| collection.get(board).is_none())
            .collect()
    }
}
