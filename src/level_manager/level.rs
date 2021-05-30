use bbecs::data_types::point::Point;

pub struct Level {
    pub start: Point,
    pub width: f32,
    pub height: f32,
    pub floor: bool,
    pub unit_width: f32,
    pub unit_height: f32,
}

impl Level {
    pub fn new() -> Self {
        let start = Point::new(150.0, 0.0);
        let width = 1920.0;
        let height = 1080.0;
        let floor = true;
        let unit_width = 70.0;
        let unit_height = 70.0;

        Self {
            start,
            width,
            height,
            floor,
            unit_width,
            unit_height,
        }
    }
}
