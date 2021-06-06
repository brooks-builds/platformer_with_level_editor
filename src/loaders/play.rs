use bbecs::{data_types::point::Point, world::World};
use eyre::{bail, Result};

use crate::{
    image_manager::ImageName,
    level_manager::{level::Level, LevelManager},
};

use super::super::level_manager::level::Entity;
use super::insert_into_world::InsertIntoWorld;

pub struct PlayLoader;

impl PlayLoader {
    fn load_camera(&self, world: &mut World, _level: &Level) -> Result<()> {
        InsertIntoWorld::new()
            .set_acceleration(Point::new(0.0, 0.0))
            .set_position(Point::new(1920.0 / 2.0, 1080.0 / 2.0))
            .set_velocity(Point::new(0.0, 0.0))
            .set_camera(true)
            .set_width(1920.0)
            .set_height(1080.0)
            .insert(world)?;
        Ok(())
    }

    fn load_entities(&self, world: &mut World, level: &Level) -> Result<()> {
        for (grid_coordinate, entity) in level.map.iter() {
            match entity {
                Entity::Platform => {
                    InsertIntoWorld::new()
                        .set_height(level.unit_height)
                        .set_position(level.grid_coordinate_to_point(grid_coordinate))
                        .set_width(level.unit_width)
                        .set_platform()
                        .set_image_name(ImageName::GrassMiddle.to_string())
                        .insert(world)?;
                }
                Entity::Player => {
                    InsertIntoWorld::new()
                        .set_position(level.grid_coordinate_to_point(grid_coordinate))
                        .set_velocity(Point::new(0.0, 0.0))
                        .set_acceleration(Point::new(0.0, 0.0))
                        .set_affected_by_gravity(true)
                        .set_height(112.0)
                        .set_width(70.0)
                        .set_image_name(ImageName::Player.to_string())
                        .set_player()
                        .set_state(crate::names::entity_states::EntityStates::Falling)
                        .insert(world)?;
                }
                Entity::End => {
                    InsertIntoWorld::new()
                        .set_height(level.unit_height)
                        .set_position(level.grid_coordinate_to_point(grid_coordinate))
                        .set_width(level.unit_width)
                        .set_end()
                        .set_image_name(ImageName::End.to_string())
                        .insert(world)?;
                }
            }
        }

        Ok(())
    }

    pub fn load(
        &mut self,
        world: &mut bbecs::world::World,
        level_manager: &LevelManager,
        level_name: String,
    ) -> eyre::Result<()> {
        let level = if let Some(level) = level_manager.get_level(&level_name) {
            level
        } else {
            bail!("Could not find a level with name {}", level_name);
        };

        self.load_camera(world, level)?;
        self.load_entities(world, level)?;
        Ok(())
    }
}
