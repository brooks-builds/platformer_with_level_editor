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
        let (accelerations,) = query!(world, query, ComponentNames::Acceleration.as_ref());

        for acceleration in accelerations {
            let wrapped_acceleration: &DataWrapper<Point> = acceleration.cast()?;
            wrapped_acceleration.borrow_mut().y += gravity;
        }
        Ok(())
    }
}
