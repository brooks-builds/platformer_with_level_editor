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

pub struct UpdateForcesSystem;

impl UpdateForcesSystem {
    pub fn run(&self, world: &World) -> Result<()> {
        let query;
        let (accelerations, velocities, positions) = query!(
            world,
            query,
            ComponentNames::Acceleration.as_ref(),
            ComponentNames::Velocity.as_ref(),
            ComponentNames::Position.as_ref()
        );
        let max_velocity = get_f32(world, ResourceNames::MaxVelocity.as_ref());

        for (index, acceleration) in accelerations.iter().enumerate() {
            let acceleration: &DataWrapper<Point> = acceleration.cast()?;
            let velocity: &DataWrapper<Point> = velocities[index].cast()?;
            let position: &DataWrapper<Point> = positions[index].cast()?;

            *velocity.borrow_mut() += *acceleration.borrow();
            velocity.borrow_mut().clamp(max_velocity, -max_velocity);
            *position.borrow_mut() += *velocity.borrow();
            acceleration.borrow_mut().multiply_scalar(0.0);
        }
        Ok(())
    }
}
