use bbecs::data_types::point::Point;

use crate::level_manager::LevelManager;

use super::{insert_into_world::InsertIntoWorld, Loader};

pub struct PlayLoader;

impl Loader for PlayLoader {
    fn load(
        &mut self,
        world: &mut bbecs::world::World,
        _context: &mut ggez::Context,
        level_manager: &LevelManager,
    ) -> eyre::Result<()> {
        dbg!("inserting play into world");
        let level = level_manager.get_level();

        InsertIntoWorld::new()
            .set_position(level.start)
            .set_velocity(Point::new(0.0, 0.0))
            .set_acceleration(Point::new(0.0, 0.0))
            .insert(world)?;
        Ok(())
    }
}
