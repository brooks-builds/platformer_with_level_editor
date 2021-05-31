use bbecs::{data_types::point::Point, world::World};
use eyre::Result;

use crate::{
    image_manager::ImageName,
    level_manager::{level::Level, LevelManager},
};

use super::{insert_into_world::InsertIntoWorld, Loader};

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

    fn load_player(&self, world: &mut World, level: &Level) -> Result<()> {
        InsertIntoWorld::new()
            .set_position(level.start)
            .set_velocity(Point::new(0.0, 0.0))
            .set_acceleration(Point::new(0.0, 0.0))
            .set_affected_by_gravity(true)
            .set_height(111.0)
            .set_width(70.0)
            .set_image_name(ImageName::Player.to_string())
            .set_player()
            .insert(world)?;

        Ok(())
    }

    fn load_floor(&self, world: &mut World, level: &Level) -> Result<()> {
        if !level.floor {
            return Ok(());
        }

        let floor_count = level.width / level.unit_width;

        for count in 0..=floor_count as u32 {
            let middle_point = Point::new(
                (level.unit_width * count as f32) + level.unit_width / 2.0,
                level.height - level.unit_height / 2.0,
            );

            InsertIntoWorld::new()
                .set_height(level.unit_height)
                .set_position(middle_point)
                .set_width(level.unit_width)
                .set_platform()
                .set_image_name(ImageName::GrassMiddle.to_string())
                .insert(world)?;
        }

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
        self.load_floor(world, level)?;
        Ok(())
    }
}
