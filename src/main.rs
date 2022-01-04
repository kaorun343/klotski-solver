mod block;
mod board;
mod bounding_rectangle;
mod direction;

use block::Block;
use board::Board;
use fnv::FnvHashSet;

fn main() {
    let mut collection = FnvHashSet::default();
    let blocks = vec![
        Block::new("父親", [0, 0], &[1, 2]),
        Block::with_goal("娘", [1, 0], &[2, 2], &[1, 3]),
        Block::new("母親", [3, 0], &[1, 2]),
        Block::new("祖父", [0, 2], &[1, 2]),
        Block::new("兄弟", [1, 2], &[2, 1]),
        Block::new("祖母", [3, 2], &[1, 2]),
        Block::new("華道", [1, 3], &[1, 1]),
        Block::new("茶道", [2, 3], &[1, 1]),
        Block::new("和裁", [0, 4], &[1, 1]),
        Block::new("書道", [3, 4], &[1, 1]),
    ];

    let board = Board::new([4, 5], blocks);
    collection.insert(board.clone());

    let mut boards = FnvHashSet::default();
    boards.insert(board);

    let mut idx = 0u64;

    loop {
        if let Some(board) = boards.iter().find(|board| board.is_finished()) {
            println!("{:?}", board);
            return;
        } else {
            boards = Board::run(boards, &collection);
            collection.extend(boards.iter().cloned());
            idx += 1;

            println!(
                "idx: {}, current: {}, total: {}",
                idx,
                boards.len(),
                collection.len()
            );
        }
    }
}
