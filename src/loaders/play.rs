use bbecs::{data_types::point::Point, world::World};
use eyre::Result;

use crate::level_manager::{level::Level, LevelManager};

use super::{insert_into_world::InsertIntoWorld, Loader};

pub struct PlayLoader;

impl PlayLoader {
    fn load_camera(&self, world: &mut World, level: &Level) -> Result<()> {
        InsertIntoWorld::new()
            .set_acceleration(Point::new(0.0, 0.0))
            .set_position(Point::new(1920.0 / 2.0, 1080.0 / 2.0))
            .set_velocity(Point::new(1.0, 0.0))
            .set_camera(true)
            .set_width(1920.0)
            .set_height(1080.0)
            .insert(world)?;
        Ok(())
    }

    fn load_player(&self, world: &mut World, level: &Level) -> Result<()> {
        InsertIntoWorld::new()
            .set_position(level.start)
            .set_velocity(Point::new(0.0, 0.0))
            .set_acceleration(Point::new(0.0, 0.0))
            .set_affected_by_gravity(true)
            .insert(world)?;

        Ok(())
    }
}

impl Loader for PlayLoader {
    fn load(
        &mut self,
        world: &mut bbecs::world::World,
        _context: &mut ggez::Context,
        level_manager: &LevelManager,
    ) -> eyre::Result<()> {
        let level = level_manager.get_level();

        self.load_player(world, level)?;
        self.load_camera(world, level)?;
        Ok(())
    }
}
