use bbecs::{data_types::point::Point, world::World};
use eyre::{bail, Result};

use crate::{helpers::query_end::query_end, image_manager::ImageName};

use super::insert_into_world::InsertIntoWorld;

pub struct WonLoader;

impl WonLoader {
    pub fn run(&self, world: &mut World) -> Result<()> {
        let end = if let Some(end) = query_end(&world)? {
            end
        } else {
            bail!("Could not find end in the world");
        };
        let width = 800.0;
        let height = 600.0;
        let position = Point::new(
            end.position.borrow().x,
            end.position.borrow().y - height / 2.0,
        );
        InsertIntoWorld::new()
            .set_image_name(ImageName::Won.to_string())
            .set_position(position)
            .set_width(width)
            .set_height(height)
            .insert(world)?;
        Ok(())
    }
}
