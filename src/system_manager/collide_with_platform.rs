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
        let mut is_standing = false;

        for platform in platforms {
            if self.is_colliding(&platform, &*player.position.borrow(), &player) {
                if self.was_player_above(&platform, &player) {
                    player.velocity.unwrap().borrow_mut().y = 0.0;
                    player.position.borrow_mut().y = platform.top() - player.height / 2.0;
                    player.set_state(EntityStates::Standing);
                    is_standing = true;
                } else if self.was_player_below(&platform, &player)
                    && !self.was_player_left(&platform, &player)
                    && !self.was_player_right(&platform, &player)
                {
                    player.velocity.unwrap().borrow_mut().y = 0.0;
                    player.position.borrow_mut().y = platform.bottom() + player.height / 2.0;
                    is_standing = false;
                }
                if self.was_player_left(&platform, &player) {
                    player.velocity.unwrap().borrow_mut().x = 0.0;
                    player.position.borrow_mut().x = platform.left() - player.width / 2.0;
                } else if self.was_player_right(&platform, &player) {
                    player.velocity.unwrap().borrow_mut().x = 0.0;
                    player.position.borrow_mut().x = platform.right() + player.width / 2.0;
                }
            } else if self.is_touching(&platform, &player) {
                player.set_state(EntityStates::Standing);
                is_standing = true;
            }
        }

        if !is_standing {
            player.set_state(EntityStates::Falling);
        }

        Ok(())
    }

    fn is_colliding(
        &self,
        platform: &GameEntity,
        _player_position: &Point,
        player: &GameEntity,
    ) -> bool {
        let player_top_left = player.top_left();
        let platform_top_left = platform.top_left();

        player_top_left.x < platform_top_left.x + platform.width
            && player_top_left.x + player.width > platform_top_left.x
            && player_top_left.y < platform_top_left.y + platform.height
            && player_top_left.y + player.height > platform_top_left.y
    }

    fn was_player_above(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().y + player.height / 2.0 <= platform.top()
    }

    fn was_player_below(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().y - player.height / 2.0 <= platform.bottom()
    }

    fn was_player_left(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().x + player.width / 2.0 <= platform.left()
    }

    fn was_player_right(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        player.calculate_previous_position().unwrap().x - player.width / 2.0 >= platform.right()
    }

    #[allow(clippy::suspicious_operation_groupings)]
    fn is_touching(&self, platform: &GameEntity, player: &GameEntity) -> bool {
        let distance = (platform.top() - player.bottom()).abs();
        let error_delta = 0.0000000000001;
        distance < error_delta
            && player.right() > platform.left()
            && player.left() < platform.right()
    }
}
