use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    query,
    world::{DataWrapper, World},
};
use eyre::Result;

use crate::{helpers::game_entity::GameEntity, names::component_names::ComponentNames};

pub fn query_player(world: &World) -> Result<Option<GameEntity>> {
    let query;
    let (widths, heights, positions, players, velocities) = query!(
        world,
        query,
        ComponentNames::Width.as_ref(),
        ComponentNames::Height.as_ref(),
        ComponentNames::Position.as_ref(),
        ComponentNames::Player.as_ref(),
        ComponentNames::Velocity.as_ref()
    );

    if players.is_empty() {
        return Ok(None);
    }

    let wrapped_width: &DataWrapper<f32> = widths[0].cast()?;
    let width = *wrapped_width.borrow();
    let wrapped_height: &DataWrapper<f32> = heights[0].cast()?;
    let height = *wrapped_height.borrow();
    let position: &DataWrapper<Point> = positions[0].cast()?;
    let velocity: &DataWrapper<Point> = velocities[0].cast()?;

    if widths.is_empty() {
        Ok(None)
    } else {
        Ok(Some(GameEntity {
            width,
            height,
            position,
            velocity: Some(velocity),
        }))
    }
}
