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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_inside_of() {
        let board = BoundingRectangle::new(&[0, 0], &[4, 5]);

        let block = BoundingRectangle::new(&[0, 0], &[1, 2]);
        assert!(block.is_inside_of(&board));

        let block = BoundingRectangle::new(&[3, 0], &[1, 2]);
        assert!(block.is_inside_of(&board));

        let block = BoundingRectangle::new(&[3, 0], &[2, 2]);
        assert!(!block.is_inside_of(&board));

        let block = BoundingRectangle::new(&[1, 4], &[2, 2]);
        assert!(!block.is_inside_of(&board));
    }

    #[test]
    fn test_is_far_from() {
        let block1 = BoundingRectangle::new(&[0, 0], &[1, 2]);
        let block2 = BoundingRectangle::new(&[3, 0], &[1, 2]);
        assert!(block1.is_far_from(&block2));

        let block1 = BoundingRectangle::new(&[0, 0], &[1, 2]);
        let block2 = BoundingRectangle::new(&[1, 0], &[2, 2]);
        assert!(block1.is_far_from(&block2));

        let block1 = BoundingRectangle::new(&[1, 0], &[1, 2]);
        let block2 = BoundingRectangle::new(&[1, 0], &[2, 2]);
        assert!(!block1.is_far_from(&block2));

        let block1 = BoundingRectangle::new(&[2, 0], &[1, 2]);
        let block2 = BoundingRectangle::new(&[1, 0], &[2, 2]);
        assert!(!block1.is_far_from(&block2));
    }
}
