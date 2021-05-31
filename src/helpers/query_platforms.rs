use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    query,
    world::{DataWrapper, World},
};
use eyre::Result;

use crate::names::component_names::ComponentNames;

use super::game_entity::GameEntity;

pub fn query_platforms(world: &World) -> Result<Vec<GameEntity>> {
    let query;
    let (platforms, positions, widths, heights) = query!(
        world,
        query,
        ComponentNames::Platform.as_ref(),
        ComponentNames::Position.as_ref(),
        ComponentNames::Width.as_ref(),
        ComponentNames::Height.as_ref()
    );
    let mut game_entities = vec![];

    if platforms.is_empty() {
        return Ok(game_entities);
    }

    for (index, _platform) in platforms.iter().enumerate() {
        let wrapped_position: &DataWrapper<Point> = positions[index].cast()?;
        let wrapped_width: &DataWrapper<f32> = widths[index].cast()?;
        let wrapped_height: &DataWrapper<f32> = heights[index].cast()?;
        let game_entity = GameEntity {
            position: wrapped_position,
            width: *wrapped_width.borrow(),
            height: *wrapped_height.borrow(),
            velocity: None,
        };

        game_entities.push(game_entity);
    }

    Ok(game_entities)
}
