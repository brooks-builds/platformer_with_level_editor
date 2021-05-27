use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    query,
    world::{DataWrapper, World},
};
use eyre::Result;

use crate::{
    helpers::get_resource::get_f32,
    names::{component_names::ComponentNames, resource_names::ResourceNames},
};

pub struct ApplyGravitySystem;

impl ApplyGravitySystem {
    pub fn run(&self, world: &World) -> Result<()> {
        let gravity = get_f32(world, ResourceNames::Gravity.as_ref());
        let query;
        let (positions,) = query!(world, query, ComponentNames::Position.as_ref());

        for position in positions {
            let wrapped_position: &DataWrapper<Point> = position.cast()?;
            wrapped_position.borrow_mut().y += gravity;
        }
        Ok(())
    }
}
