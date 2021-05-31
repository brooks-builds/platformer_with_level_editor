use bbecs::world::World;
use eyre::Result;

use crate::helpers::{query_platforms::query_platforms, query_player::query_player};

pub struct CollideWithPlatform;

impl CollideWithPlatform {
    pub fn run(&self, world: &World) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };
        let platforms = query_platforms(world)?;
        let borrowed_player_position = player.position.borrow();

        for platform in platforms {
            let player_right = borrowed_player_position.x + player.width / 2.0;
            let platform_left = platform.position.borrow().x - platform.width / 2.0;
            let player_left = borrowed_player_position.x - player.width / 2.0;
            let platform_right = platform.position.borrow().x + platform.width / 2.0;
            let player_bottom = borrowed_player_position.y + player.height / 2.0;
            let platform_top = platform.position.borrow().y - platform.height / 2.0;

            if player_right > platform_left
                && player_left < platform_right
                && player_bottom > platform_top
            {
                let mut player_velocity = player.velocity.unwrap().borrow_mut();
                player_velocity.y = 0.0;
            }
        }

        Ok(())
    }
}
