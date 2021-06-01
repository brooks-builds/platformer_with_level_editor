use bbecs::{data_types::point::Point, world::World};
use eyre::Result;
use ggez::{
    graphics::{self, Scale, Text, TextFragment, WHITE},
    Context,
};

use crate::{
    helpers::get_resource::get_f32, level_manager::LevelManager,
    names::resource_names::ResourceNames,
};

use super::{insert_into_world::InsertIntoWorld, Loader};

pub struct SettingsLoader;

impl SettingsLoader {
    fn create_title(&self, world: &mut World, context: &mut Context) -> Result<()> {
        let font_size = get_f32(world, ResourceNames::TitleFontSize.as_ref());
        let title_fragment = TextFragment::new("Settings")
            .color(WHITE)
            .scale(Scale::uniform(font_size));
        let title = Text::new(title_fragment);
        let screen_coordinates = graphics::screen_coordinates(context);
        let position = Point::new(
            screen_coordinates.w / 2.0 - title.width(context) as f32 / 2.0,
            5.0,
        );

        InsertIntoWorld::new()
            .set_position(position)
            .set_text(title)
            .insert(world)?;
        Ok(())
    }
}

impl Loader for SettingsLoader {
    fn load(
        &mut self,
        world: &mut bbecs::world::World,
        context: &mut ggez::Context,
        _level_manager: &LevelManager,
    ) -> eyre::Result<()> {
        self.create_title(world, context)?;
        Ok(())
    }
}
