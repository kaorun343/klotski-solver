#[derive(Debug)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

pub static ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Top,
    Direction::Right,
    Direction::Bottom,
];
