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
        let (accelerations, affected_by_gravity) = query!(
            world,
            query,
            ComponentNames::Acceleration.as_ref(),
            ComponentNames::AffectedByGravity.as_ref()
        );

        for (index, acceleration) in accelerations.iter().enumerate() {
            let wrapped_acceleration: &DataWrapper<Point> = acceleration.cast()?;
            let wrapped_affected_by_gravity: &DataWrapper<bool> =
                affected_by_gravity[index].cast()?;

            if *wrapped_affected_by_gravity.borrow() {
                wrapped_acceleration.borrow_mut().y += gravity;
            }
        }
        Ok(())
    }
}
