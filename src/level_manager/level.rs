use bbecs::data_types::point::Point;

pub struct Level {
    pub start: Point,
}

impl Level {
    pub fn new() -> Self {
        let start = Point::new(1.0, 0.0);

        Self { start }
    }
}
