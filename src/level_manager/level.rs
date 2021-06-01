use std::collections::HashMap;

use bbecs::data_types::point::Point;
use rand::{thread_rng, Rng};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct GridCoordinate {
    pub x: u32,
    pub y: u32,
}

impl GridCoordinate {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub enum Entity {
    Platform,
    Player,
}

pub struct Level {
    pub start: GridCoordinate,
    pub width: u32,
    pub height: u32,
    pub unit_width: f32,
    pub unit_height: f32,
    pub map: HashMap<GridCoordinate, Entity>,
    pub name: String,
}

impl Level {
    pub fn new(name: String) -> Self {
        let start = GridCoordinate::new(2, 0);
        let width = 100;
        let height = 15;
        let unit_width = 70.0;
        let unit_height = 70.0;
        let mut map = HashMap::new();
        let mut rng = thread_rng();
        map.insert(GridCoordinate::new(2, 0), Entity::Player);

        for y in 0..height {
            let mut x = rng.gen_range(0..10);
            if x == 2 {
                x = 1
            }
            map.insert(GridCoordinate::new(x, y), Entity::Platform);
        }

        Self {
            start,
            width,
            height,
            unit_width,
            unit_height,
            map,
            name,
        }
    }

    pub fn grid_coordinate_to_point(&self, grid_coordinate: &GridCoordinate) -> Point {
        Point::new(
            grid_coordinate.x as f32 * self.unit_width + self.unit_width / 2.0,
            grid_coordinate.y as f32 * self.unit_height + self.unit_height / 2.0,
        )
    }

    pub fn add_floor(&mut self) {
        for count in 0..self.width {
            self.map.insert(
                GridCoordinate::new(count, self.height - 1),
                Entity::Platform,
            );
        }
    }
}
