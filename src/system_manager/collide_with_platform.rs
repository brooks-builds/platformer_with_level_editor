use bbecs::{data_types::point::Point, world::World};
use eyre::Result;

use crate::{
    helpers::{
        game_entity::GameEntity, query_platforms::query_platforms, query_player::query_player,
    },
    names::entity_states::EntityStates,
};

pub struct CollideWithPlatform;

impl CollideWithPlatform {
    pub fn run(&self, world: &World) -> Result<()> {
        let mut player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };
        let platforms = query_platforms(world)?;

        for platform in platforms {
            if self.is_colliding(&platform, &*player.position.borrow(), &player) {
                if self.was_player_above(&platform, &player) {
                    player.velocity.unwrap().borrow_mut().y = 0.0;
                    player.position.borrow_mut().y = platform.top() - player.height / 2.0;
                    player.set_state(EntityStates::Standing);
                }
                if self.was_player_left(&platform, &player) {
                    dbg!("colliding right");
                    player.velocity.unwrap().borrow_mut().x = 0.0;
                    player.position.borrow_mut().x = platform.left() - player.width / 2.0 - 1.0;
                } else if self.was_player_right(&platform, &player) {
                    dbg!("colliding left");
                    player.velocity.unwrap().borrow_mut().x = 0.0;
                    player.position.borrow_mut().x = platform.right() + player.width / 2.0 + 1.0;
                }
            } else {
                player.set_state(EntityStates::Falling);
            }
        }

        Ok(())
    }

    fn is_colliding(
        &self,
        platform: &GameEntity,
        player_position: &Point,
        player: &GameEntity,
    ) -> bool {
        let player_right = player_position.x + player.width / 2.0;
        let platform_left = platform.position.borrow().x - platform.width / 2.0;
        let player_left = player_position.x - player.width / 2.0;
        let platform_right = platform.position.borrow().x + platform.width / 2.0;
        let player_bottom = player_position.y + player.height / 2.0;
        let platform_top = platform.position.borrow().y - platform.height / 2.0;
        let player_top = player_position.y - player.height / 2.0;
        let platform_bottom = platform.position.borrow().y + platform.height / 2.0;

        player_bottom > platform_top
            && player_top < platform_bottom
            && player_left < platform_right
            && player_right > platform_left
    }

    fn was_player_above(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().y + player.height / 2.0 <= platform.top()
    }

    fn was_player_left(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().x + player.width / 2.0 <= platform.left()
    }

    fn was_player_right(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().x - player.width / 2.0 >= platform.right()
    }
}
