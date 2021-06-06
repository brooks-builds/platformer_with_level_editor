use bbecs::{data_types::point::Point, world::World};
use eyre::Result;

use crate::{
    helpers::{get_resource::get_f32, query_player::query_player},
    names::{entity_states::EntityStates, resource_names::ResourceNames},
};

pub struct ApplyGravitySystem;

impl ApplyGravitySystem {
    pub fn run(&self, world: &World) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };
        let gravity = Point::new(0.0, get_f32(world, ResourceNames::Gravity.as_ref()));

        if player.state().unwrap() == EntityStates::Falling {
            *player.acceleration.unwrap().borrow_mut() += gravity;
        }

        Ok(())
    }
}
