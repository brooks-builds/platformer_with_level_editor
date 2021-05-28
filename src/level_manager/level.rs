use bbecs::data_types::point::Point;

pub struct Level {
    pub start: Point,
    pub width: f32,
    pub height: f32,
}

impl Level {
    pub fn new() -> Self {
        let start = Point::new(2000.0, 0.0);
        let width = 10_000.0;
        let height = 2_000.0;

        Self {
            start,
            width,
            height,
        }
    }
}
