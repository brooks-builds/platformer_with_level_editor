use bbecs::world::World;
use eyre::Result;

use crate::{
    helpers::{get_resource::get_f32, query_player::query_player},
    names::resource_names::ResourceNames,
};

pub struct ApplyFrictionSystem;

impl ApplyFrictionSystem {
    pub fn run(&self, world: &World) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };

        // if player.state().unwrap() == EntityStates::Standing {
        let mut friction_force = *player.velocity.unwrap().borrow();
        let coefficient_of_friction = get_f32(world, ResourceNames::Friction.as_ref());

        friction_force.multiply_scalar(-1.0);
        friction_force.normalize();
        friction_force.multiply_scalar(coefficient_of_friction);

        *player.acceleration.unwrap().borrow_mut() += friction_force;
        // }
        Ok(())
    }
}
