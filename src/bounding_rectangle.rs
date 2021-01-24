pub struct BoundingRectangle {
    left: i64,
    top: i64,
    right: i64,
    bottom: i64,
}

impl BoundingRectangle {
    pub fn new(&[x, y]: &[i64; 2], &[w, h]: &[i64; 2]) -> Self {
        Self {
            left: x,
            top: y,
            right: x + w - 1,
            bottom: y + h - 1,
        }
    }

    pub fn is_inside_of(&self, other: &Self) -> bool {
        self.left >= other.left
            && self.top >= other.top
            && self.right <= other.right
            && self.bottom <= other.bottom
    }

    pub fn is_far_from(&self, other: &Self) -> bool {
        self.right < other.left
            || self.left > other.right
            || self.bottom < other.top
            || self.top > other.bottom
    }
}
