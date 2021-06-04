use std::collections::HashMap;

use bbecs::data_types::point::Point;

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
    End,
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
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let start = GridCoordinate::new(2, 0);
        let unit_width = 70.0;
        let unit_height = 70.0;
        let mut map = HashMap::new();
        map.insert(GridCoordinate::new(2, 0), Entity::Player);

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

    pub fn add_end(&mut self, x: u32, y: u32) {
        let grid_coordinate = GridCoordinate::new(x, y);
        let entity = Entity::End;
        self.map.insert(grid_coordinate, entity);
    }
}
