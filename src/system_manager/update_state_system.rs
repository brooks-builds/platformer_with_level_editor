use bbecs::world::World;
use eyre::Result;

use crate::helpers::{query_platforms::query_platforms, query_player::query_player};

pub struct UpdateStateSystem;

impl UpdateStateSystem {
    pub fn run(&self, world: &World) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };

        let platforms = query_platforms(world)?;
        Ok(())
    }
}
