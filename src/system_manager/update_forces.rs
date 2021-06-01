use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    query,
    world::{DataWrapper, World},
};
use eyre::Result;

use crate::names::component_names::ComponentNames;

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

        for (index, acceleration) in accelerations.iter().enumerate() {
            let acceleration: &DataWrapper<Point> = acceleration.cast()?;
            let velocity: &DataWrapper<Point> = velocities[index].cast()?;
            let position: &DataWrapper<Point> = positions[index].cast()?;

            *velocity.borrow_mut() += *acceleration.borrow();
            *position.borrow_mut() += *velocity.borrow();
            acceleration.borrow_mut().multiply_scalar(0.0);
        }
        Ok(())
    }
}
