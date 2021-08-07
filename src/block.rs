use crate::{bounding_rectangle::BoundingRectangle, direction::Direction};

#[derive(Clone, Debug)]
pub struct Block {
    name: String,
    position: [i64; 2],
    size: [i64; 2],
    goal: Option<[i64; 2]>,
}

impl<'a> Into<BoundingRectangle> for &'a Block {
    fn into(self) -> BoundingRectangle {
        BoundingRectangle::new(&self.position, &self.size)
    }
}

impl Block {
    pub fn new(name: &str, position: [i64; 2], size: [i64; 2]) -> Self {
        Block {
            name: name.to_owned(),
            position,
            size,
            goal: None,
        }
    }

    pub fn with_goal(name: &str, position: [i64; 2], size: [i64; 2], goal: [i64; 2]) -> Self {
        Block {
            name: name.to_owned(),
            position,
            size,
            goal: Some(goal),
        }
    }

    pub fn move_to(&self, direction: &Direction) -> Self {
        Block {
            name: self.name.clone(),
            position: self.next_position(direction),
            size: self.size.clone(),
            goal: self.goal.clone(),
        }
    }

    fn next_position(&self, direction: &Direction) -> [i64; 2] {
        let &[x, y] = &self.position;
        use Direction::*;
        match direction {
            Left => [x - 1, y],
            Top => [x, y - 1],
            Right => [x + 1, y],
            Bottom => [x, y + 1],
        }
    }

    pub fn is_at_goal(&self) -> bool {
        match &self.goal {
            Some(ref goal) => goal == &self.position,
            None => true,
        }
    }
}
