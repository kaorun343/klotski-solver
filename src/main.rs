mod board;

use board::{Board, Shape};
use fnv::FnvHashSet;

fn main() {
    use Shape::*;

    let board = Board::new(&[
        (Vertical, [0, 0]),   // 父親
        (Square, [1, 0]),     // 娘
        (Vertical, [3, 0]),   // 母親
        (Vertical, [0, 2]),   // 祖父
        (Horizontal, [1, 2]), // 兄弟
        (Vertical, [3, 2]),   // 祖母
        (Small, [1, 3]),      // 華道
        (Small, [2, 3]),      // 茶道
        (Small, [0, 4]),      // 和裁
        (Small, [3, 4]),      // 書道
    ])
    .canonical();

    let mut collection = FnvHashSet::default();
    collection.insert(board);

    let mut boards = vec![board];

    let mut idx = 0u64;

    loop {
        if let Some(board) = boards.iter().find(|board| board.is_finished()) {
            print!("{}", board);
            return;
        } else {
            boards = Board::run(&boards, &mut collection);
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
