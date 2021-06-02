use bbecs::data_types::point::Point;
use ggez::graphics::Text;

use super::insert_into_world::InsertIntoWorld;

pub struct EditScene;

impl EditScene {
    pub fn load(
        &mut self,
        world: &mut bbecs::world::World,
        _context: &mut ggez::Context,
        _level_manager: &crate::level_manager::LevelManager,
    ) -> eyre::Result<()> {
        InsertIntoWorld::new()
            .set_position(Point::new(0.0, 0.0))
            .set_text(Text::new("Edit Mode"))
            .insert(world)
    }
}
