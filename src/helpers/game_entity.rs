use bbecs::{data_types::point::Point, world::DataWrapper};

pub struct GameEntity<'a> {
    pub width: f32,
    pub height: f32,
    pub position: &'a DataWrapper<Point>,
    pub velocity: Option<&'a DataWrapper<Point>>,
    pub acceleration: Option<&'a DataWrapper<Point>>,
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
}
