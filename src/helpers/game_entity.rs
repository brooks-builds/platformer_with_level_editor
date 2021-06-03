use bbecs::{data_types::point::Point, world::DataWrapper};

use crate::names::entity_states::EntityStates;

pub struct GameEntity<'a> {
    pub width: f32,
    pub height: f32,
    pub position: &'a DataWrapper<Point>,
    pub velocity: Option<&'a DataWrapper<Point>>,
    pub acceleration: Option<&'a DataWrapper<Point>>,
    pub state: Option<&'a DataWrapper<String>>,
}

impl<'a> GameEntity<'a> {
    pub fn calculate_previous_position(&self) -> Option<Point> {
        let mut position = *self.position.borrow();
        position.x -= self.velocity?.borrow().x;
        position.y -= self.velocity?.borrow().y;
        Some(position)
    }

    pub fn top(&self) -> f32 {
        self.position.borrow().y - self.height / 2.0
    }

    pub fn left(&self) -> f32 {
        self.position.borrow().x - self.width / 2.0
    }

    pub fn right(&self) -> f32 {
        self.position.borrow().x + self.width / 2.0
    }

    pub fn bottom(&self) -> f32 {
        self.position.borrow().y + self.height / 2.0
    }

    pub fn state(&self) -> Option<EntityStates> {
        Some(EntityStates::from_str(&*self.state?.borrow()))
    }

    pub fn set_state(&mut self, new_state: EntityStates) -> Option<()> {
        *self.state?.borrow_mut() = new_state.to_string();
        Some(())
    }

    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.right() > other.left()
            && self.left() < other.right()
            && self.bottom() > other.top()
            && self.top() < other.bottom()
    }
}
