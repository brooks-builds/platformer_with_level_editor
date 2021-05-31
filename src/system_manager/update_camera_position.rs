use bbecs::world::World;
use eyre::Result;

use crate::helpers::{query_camera::query_camera, query_player::query_player};

pub struct UpdateCameraPosition;

impl UpdateCameraPosition {
    pub fn run(&self, world: &World) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };
        let camera = if let Some(camera) = query_camera(world)? {
            camera
        } else {
            return Ok(());
        };

        camera.position.borrow_mut().x = player.position.borrow().x;
        camera.position.borrow_mut().y = player.position.borrow().y - 415.0;
        Ok(())
    }
}
