use bbecs::data_types::point::Point;
use bbecs::get_resource;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{Font, Scale, Text, TextFragment, WHITE};
use ggez::Context;

use super::insert_into_world::InsertIntoWorld;
use super::Loader;

pub struct SelectLevelLoader;

impl SelectLevelLoader {
    fn create_title(&self, world: &mut World, _context: &mut Context) -> Result<()> {
        let position = Point::new(5.0, 5.0);
        let title_fragment = TextFragment::new("Select Level")
            .color(WHITE)
            .font(Font::default())
            .scale(Scale::uniform(self.get_title_font_size(&world)?));
        let title_text = Text::new(title_fragment);

        InsertIntoWorld::new()
            .set_position(position)
            .set_text(title_text)
            .insert(world)?;
        Ok(())
    }

    fn get_title_font_size(&self, world: &World) -> Result<f32> {
        let resource;
        get_resource!(
            resource,
            world,
            crate::names::resource_names::ResourceNames::TitleFontSize.as_ref()
        );
        Ok(*resource)
    }
}

impl Loader for SelectLevelLoader {
    fn load(&mut self, world: &mut World, context: &mut Context) -> Result<()> {
        self.create_title(world, context)?;
        Ok(())
    }
}
