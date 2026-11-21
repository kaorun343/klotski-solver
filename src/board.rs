use fnv::FnvHashSet;
use std::fmt;

pub const WIDTH: i32 = 4;
pub const HEIGHT: i32 = 5;

/// 1マスあたりのビット数。各マスに「そこを左上とするブロックの形」を書く。
const BITS: u32 = 3;
const CELL_MASK: u64 = (1 << BITS) - 1;

/// ブロックの形。値はビットボード上のコード(0 は空きマス)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// 2×2(娘)
    Square = 1,
    /// 1×2(縦長)
    Vertical = 2,
    /// 2×1(横長)
    Horizontal = 3,
    /// 1×1
    Small = 4,
}

impl Shape {
    #[inline]
    fn from_code(code: u64) -> Option<Self> {
        use Shape::*;
        match code {
            1 => Some(Square),
            2 => Some(Vertical),
            3 => Some(Horizontal),
            4 => Some(Small),
            _ => None,
        }
    }

    /// [幅, 高さ]
    #[inline]
    fn size(self) -> [i32; 2] {
        use Shape::*;
        match self {
            Square => [2, 2],
            Vertical => [1, 2],
            Horizontal => [2, 1],
            Small => [1, 1],
        }
    }

    /// 左上が (x, y) のとき、このブロックが占めるマスのビットマスク。
    #[inline]
    fn cells(self, x: i32, y: i32) -> u32 {
        let [w, h] = self.size();
        let row = (1u32 << w) - 1;
        (0..h).fold(0, |mask, dy| mask | row << ((y + dy) * WIDTH + x))
    }

    #[inline]
    fn fits(self, x: i32, y: i32) -> bool {
        let [w, h] = self.size();
        x >= 0 && y >= 0 && x + w <= WIDTH && y + h <= HEIGHT
    }

    fn glyph(self) -> char {
        use Shape::*;
        match self {
            Square => '娘',
            Vertical => '縦',
            Horizontal => '横',
            Small => '小',
        }
    }
}

const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

/// 娘のゴール位置(左上)。
const GOAL: [i32; 2] = [1, 3];

#[inline]
fn shift(x: i32, y: i32) -> u32 {
    (y * WIDTH + x) as u32 * BITS
}

/// 盤面を u64 1個で表す。マス (x, y) の BITS ビットに、そこを左上とするブロックの形が入る。
/// 同じ形のブロックは区別しないので、この表現がそのまま正規形になる。
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Board(u64);

impl Board {
    pub fn new(blocks: &[(Shape, [i32; 2])]) -> Self {
        Board(
            blocks
                .iter()
                .map(|&(shape, [x, y])| (shape as u64) << shift(x, y))
                .sum(),
        )
    }

    /// 盤面上のブロックを (形, x, y) で列挙する。
    #[inline]
    fn blocks(self) -> impl Iterator<Item = (Shape, i32, i32)> {
        (0..WIDTH * HEIGHT).filter_map(move |cell| {
            let (x, y) = (cell % WIDTH, cell / WIDTH);
            Shape::from_code((self.0 >> shift(x, y)) & CELL_MASK).map(|shape| (shape, x, y))
        })
    }

    #[inline]
    fn occupied(self) -> u32 {
        self.blocks()
            .fold(0, |mask, (shape, x, y)| mask | shape.cells(x, y))
    }

    /// 左右を反転した盤面。
    #[inline]
    fn mirrored(self) -> Self {
        Board(self.blocks().fold(0, |bits, (shape, x, y)| {
            let [w, _] = shape.size();
            bits | (shape as u64) << shift(WIDTH - w - x, y)
        }))
    }

    /// 左右反転した盤面と同一視するための正規形(小さいほう)。
    /// ゴール位置が左右対称なので、反転しても最短手数は変わらない。
    #[inline]
    pub fn canonical(self) -> Self {
        Board(self.0.min(self.mirrored().0))
    }

    #[inline]
    pub fn is_finished(self) -> bool {
        let [x, y] = GOAL;
        Shape::from_code((self.0 >> shift(x, y)) & CELL_MASK) == Some(Shape::Square)
    }

    /// 1手(同じブロックを何マス動かしても、曲がっても1手)で行ける盤面を `out` に追加する。
    fn push_next_boards(self, out: &mut Vec<Board>) {
        let occupied = self.occupied();

        for (shape, x, y) in self.blocks() {
            let others = occupied & !shape.cells(x, y);
            let code = shape as u64;
            let base = self.0 - (code << shift(x, y));

            // このブロックだけを1マスずつ動かして、たどり着ける左上の位置を探す。
            let mut visited = 1u32 << (y * WIDTH + x);
            let mut stack = [(0, 0); (WIDTH * HEIGHT) as usize];
            stack[0] = (x, y);
            let mut len = 1;

            while len > 0 {
                len -= 1;
                let (cx, cy) = stack[len];
                for &[dx, dy] in DIRECTIONS.iter() {
                    let (nx, ny) = (cx + dx, cy + dy);
                    if !shape.fits(nx, ny) {
                        continue;
                    }
                    let bit = 1u32 << (ny * WIDTH + nx);
                    if visited & bit != 0 || shape.cells(nx, ny) & others != 0 {
                        continue;
                    }
                    visited |= bit;
                    stack[len] = (nx, ny);
                    len += 1;
                    out.push(Board(base + (code << shift(nx, ny))).canonical());
                }
            }
        }
    }

    pub fn run(boards: &[Board], collection: &mut FnvHashSet<Board>) -> Vec<Board> {
        let mut candidates = Vec::new();
        for board in boards {
            board.push_next_boards(&mut candidates);
        }
        // 未訪問なら collection に登録し、次の探索対象にする(ハッシュ計算は1回)。
        candidates.retain(|&board| collection.insert(board));
        candidates
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = [['・'; WIDTH as usize]; HEIGHT as usize];
        for (shape, x, y) in self.blocks() {
            let [w, h] = shape.size();
            for dy in 0..h {
                for dx in 0..w {
                    grid[(y + dy) as usize][(x + dx) as usize] = shape.glyph();
                }
            }
        }
        for row in grid.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cells() {
        assert_eq!(Shape::Small.cells(0, 0), 0b1);
        assert_eq!(Shape::Horizontal.cells(1, 0), 0b110);
        assert_eq!(Shape::Vertical.cells(0, 0), 0b1_0001);
        assert_eq!(Shape::Square.cells(2, 3), 0b1100_1100 << 12);
    }

    #[test]
    fn test_fits() {
        assert!(Shape::Vertical.fits(3, 0));
        assert!(!Shape::Square.fits(3, 0));
        assert!(!Shape::Square.fits(1, 4));
        assert!(!Shape::Small.fits(-1, 0));
    }

    #[test]
    fn test_mirrored() {
        let board = Board::new(&[(Shape::Square, [0, 0]), (Shape::Small, [3, 4])]);
        let mirrored = Board::new(&[(Shape::Square, [2, 0]), (Shape::Small, [0, 4])]);
        assert_eq!(board.mirrored(), mirrored);
        assert_eq!(board.mirrored().mirrored(), board);
        assert_eq!(board.canonical(), mirrored.canonical());
    }

    #[test]
    fn test_one_move_slides_any_distance() {
        // ブロックが 1×1 の1個だけなら、元の位置以外の 19 マスすべてに1手で行ける。
        let board = Board::new(&[(Shape::Small, [3, 3])]);
        let mut next = Vec::new();
        board.push_next_boards(&mut next);
        assert_eq!(next.len(), 19);

        // 縦縦小・
        // 縦縦・・  (2,0) の小は (3,0), (2,1), (3,1) へ1手で行ける(L 字も1手)。
        // 娘娘娘娘  右の娘は1マス上へ行ける。
        // 娘娘娘娘
        // 横横横横
        let blocks = |small: [i32; 2], square: [i32; 2]| {
            Board::new(&[
                (Shape::Vertical, [0, 0]),
                (Shape::Vertical, [1, 0]),
                (Shape::Small, small),
                (Shape::Square, [0, 2]),
                (Shape::Square, square),
                (Shape::Horizontal, [0, 4]),
                (Shape::Horizontal, [2, 4]),
            ])
        };
        let mut next = Vec::new();
        blocks([2, 0], [2, 2]).push_next_boards(&mut next);
        let mut expected = vec![
            blocks([3, 0], [2, 2]),
            blocks([2, 1], [2, 2]),
            blocks([3, 1], [2, 2]),
            blocks([2, 0], [2, 1]),
        ]
        .into_iter()
        .map(Board::canonical)
        .collect::<Vec<_>>();
        next.sort_by_key(|board| board.0);
        expected.sort_by_key(|board| board.0);
        assert_eq!(next, expected);
    }
}
