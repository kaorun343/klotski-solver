mod block;
mod board;
mod bounding_rectangle;
mod direction;

use block::Block;
use board::Board;

fn main() {
    let blocks = vec![
        Block::new("父親", [0, 0], [1, 2]),
        Block::new("娘", [1, 0], [2, 2]),
        Block::new("母親", [3, 0], [1, 2]),
        Block::new("祖父", [0, 2], [1, 2]),
        Block::new("兄弟", [1, 2], [1, 2]),
        Block::new("祖母", [3, 2], [1, 2]),
        Block::new("華道", [1, 3], [1, 1]),
        Block::new("茶道", [2, 3], [1, 1]),
        Block::new("和裁", [0, 4], [1, 1]),
        Block::new("書道", [3, 4], [1, 1]),
    ];
    let board = Board::new([4, 5], blocks);
    let boards = board.generate_next_states();
    println!("{:?}", boards);
}
