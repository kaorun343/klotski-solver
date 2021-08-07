use crate::{block::Block, bounding_rectangle::BoundingRectangle, direction::ALL_DIRECTIONS};

#[derive(Debug)]
pub struct Board {
    size: [i64; 2],
    blocks: Vec<Block>,
}

impl<'a> Into<BoundingRectangle> for &'a Board {
    fn into(self) -> BoundingRectangle {
        BoundingRectangle::new(&[0, 0], &self.size)
    }
}

impl Board {
    pub fn new(size: [i64; 2], blocks: Vec<Block>) -> Self {
        Board { size, blocks }
    }

    pub fn generate_next_states(&self) -> Vec<Self> {
        let blocks = &self.blocks;
        blocks
            .iter()
            .enumerate()
            .flat_map(|(block_idx, block)| {
                let board_rectangle: BoundingRectangle = self.into();

                ALL_DIRECTIONS
                    .iter()
                    .map(move |direction| block.move_to(direction))
                    .filter(move |moved_block| {
                        let moved_block_rectangle: BoundingRectangle = moved_block.into();
                        moved_block_rectangle.is_inside_of(&board_rectangle)
                            && blocks
                                .iter()
                                .enumerate()
                                .filter(|&(other_block_idx, _)| other_block_idx != block_idx)
                                .all(|(_idx, other_block)| {
                                    moved_block_rectangle.is_far_from(&other_block.into())
                                })
                    })
                    .map(move |moved_block| {
                        let blocks = blocks
                            .iter()
                            .enumerate()
                            .map(|(idx, other_block)| -> Block {
                                if idx == block_idx {
                                    moved_block.clone()
                                } else {
                                    other_block.clone()
                                }
                            })
                            .collect();

                        Self::new(self.size.clone(), blocks)
                    })
            })
            .collect()
    }

    pub fn is_finished(&self) -> bool {
        self.blocks.iter().all(Block::is_at_goal)
    }
}
