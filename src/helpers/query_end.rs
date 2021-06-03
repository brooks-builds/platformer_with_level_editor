use bbecs::{
    components::CastComponents,
    query,
    world::{DataWrapper, World},
};
use eyre::{bail, Result};

use crate::names::component_names::ComponentNames;

use super::game_entity::GameEntity;

pub fn query_end(world: &World) -> Result<Option<GameEntity>> {
    let query;
    let (heights, positions, widths, _ends) = query!(
        world,
        query,
        ComponentNames::Height.as_ref(),
        ComponentNames::Position.as_ref(),
        ComponentNames::Width.as_ref(),
        ComponentNames::End.as_ref()
    );

    if heights.is_empty() {
        return Ok(None);
    }

    if heights.len() > 1 {
        bail!("Found more than one end in the world");
    }

    let wrapped_width: &DataWrapper<f32> = widths.first().unwrap().cast()?;
    let wrapped_height: &DataWrapper<f32> = heights.first().unwrap().cast()?;

    let game_entity = GameEntity {
        width: *wrapped_width.borrow(),
        height: *wrapped_height.borrow(),
        position: positions.first().unwrap().cast()?,
        velocity: None,
        acceleration: None,
        state: None,
    };

    Ok(Some(game_entity))
}
