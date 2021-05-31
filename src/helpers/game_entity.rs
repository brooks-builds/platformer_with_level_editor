use bbecs::{data_types::point::Point, world::DataWrapper};

pub struct GameEntity<'a> {
    pub width: f32,
    pub height: f32,
    pub position: &'a DataWrapper<Point>,
    pub velocity: Option<&'a DataWrapper<Point>>,
}