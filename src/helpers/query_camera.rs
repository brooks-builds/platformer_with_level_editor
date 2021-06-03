use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    query,
    world::{DataWrapper, World},
};
use eyre::Result;

use crate::names::component_names::ComponentNames;

use super::game_entity::GameEntity;

pub fn query_camera(world: &World) -> Result<Option<GameEntity>> {
    let query;
    let (cameras, positions, widths, heights) = query!(
        world,
        query,
        ComponentNames::Camera.as_ref(),
        ComponentNames::Position.as_ref(),
        ComponentNames::Width.as_ref(),
        ComponentNames::Height.as_ref()
    );

    if cameras.is_empty() {
        return Ok(None);
    }

    let wrapped_width: &DataWrapper<f32> = widths[0].cast()?;
    let wrapped_height: &DataWrapper<f32> = heights[0].cast()?;
    let wrapped_position: &DataWrapper<Point> = positions[0].cast()?;
    let velocity = None;
    let acceleration = None;

    let game_entity = GameEntity {
        width: *wrapped_width.borrow(),
        height: *wrapped_height.borrow(),
        position: wrapped_position,
        velocity,
        acceleration,
        state: None,
    };
    Ok(Some(game_entity))
}
