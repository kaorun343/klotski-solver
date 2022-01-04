use crate::{bounding_rectangle::BoundingRectangle, direction::Direction};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    name: &'a str,
    position: [i64; 2],
    size: &'a [i64; 2],
    goal: Option<&'a [i64; 2]>,
}

impl<'a> Hash for Block<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.size.hash(state);
        self.goal.hash(state);
    }
}

impl<'a> Into<BoundingRectangle> for &'a Block<'a> {
    fn into(self) -> BoundingRectangle {
        BoundingRectangle::new(&self.position, &self.size)
    }
}

impl<'a> Block<'a> {
    pub fn new(name: &'a str, position: [i64; 2], size: &'a [i64; 2]) -> Self {
        Block {
            name,
            position,
            size,
            goal: None,
        }
    }

    pub fn with_goal(
        name: &'a str,
        position: [i64; 2],
        size: &'a [i64; 2],
        goal: &'a [i64; 2],
    ) -> Self {
        Block {
            name,
            position,
            size,
            goal: Some(goal),
        }
    }

    pub fn move_to(&self, direction: &Direction) -> Self {
        Block {
            name: self.name.clone(),
            position: self.next_position(direction),
            size: self.size,
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
        match self.goal {
            Some(goal) => goal == &self.position,
            None => true,
        }
    }
}
