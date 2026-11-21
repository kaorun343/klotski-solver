use fnv::FnvHashSet;

use crate::{block::Block, bounding_rectangle::BoundingRectangle, direction::ALL_DIRECTIONS};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board<'a> {
    size: &'a [i64; 2],
    blocks: Vec<Block<'a>>,
}

impl<'a> Board<'a> {
    #[inline]
    pub fn new(size: &'a [i64; 2], blocks: Vec<Block<'a>>) -> Self {
        Board { size, blocks }
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.blocks.iter().all(Block::is_at_goal)
    }

    /// ブロック `block_idx` を、ほかのブロックを固定したまま1マスずつ動かして
    /// たどり着けるすべての位置を返す(元の位置は含まない)。
    /// 同じブロックを何マス動かしても(曲がっても)1手と数えるため。
    fn reachable_blocks(&self, block_idx: usize) -> Vec<Block<'a>> {
        let board_rectangle = BoundingRectangle::new(&[0, 0], self.size);
        let blocks = &self.blocks;

        let mut visited = vec![blocks[block_idx].clone()];
        let mut queue = vec![blocks[block_idx].clone()];

        while let Some(block) = queue.pop() {
            for direction in ALL_DIRECTIONS.iter() {
                let moved_block = block.move_to(direction);
                let moved_block_rectangle: BoundingRectangle = (&moved_block).into();

                let is_valid = moved_block_rectangle.is_inside_of(&board_rectangle)
                    && blocks
                        .iter()
                        .enumerate()
                        .filter(|&(other_block_idx, _)| other_block_idx != block_idx)
                        .all(|(_, other_block)| {
                            moved_block_rectangle.is_far_from(&other_block.into())
                        });

                if is_valid && !visited.contains(&moved_block) {
                    visited.push(moved_block.clone());
                    queue.push(moved_block);
                }
            }
        }

        visited.remove(0);
        visited
    }

    #[inline]
    pub fn run(boards: &[Board<'a>], collection: &mut FnvHashSet<Board<'a>>) -> Vec<Board<'a>> {
        boards
            .iter()
            .flat_map(|board| {
                (0..board.blocks.len()).flat_map(move |block_idx| {
                    board
                        .reachable_blocks(block_idx)
                        .into_iter()
                        .map(move |moved_block| {
                            let mut blocks = board.blocks.clone();
                            blocks[block_idx] = moved_block;
                            Board::new(board.size, blocks)
                        })
                })
            })
            // 未訪問なら collection に登録し、次の探索対象にする(ハッシュ計算は1回)。
            .filter(|board| collection.insert(board.clone()))
            .collect()
    }
}
