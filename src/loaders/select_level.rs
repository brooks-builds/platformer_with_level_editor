use bbecs::data_types::point::Point;
use bbecs::get_resource;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::{Font, Scale, Text, TextFragment, WHITE};
use ggez::input::mouse::position;
use ggez::Context;

use crate::names::component_names::ComponentNames;

pub struct SelectLevelLoader;

impl SelectLevelLoader {
    pub fn load(&self, world: &mut World, context: &mut Context) -> Result<()> {
        self.create_title(world, context)?;
        Ok(())
    }

    fn create_title(&self, world: &mut World, context: &mut Context) -> Result<()> {
        let position = Point::new(5.0, 5.0);
        let title_fragment = TextFragment::new("Select Level")
            .color(WHITE)
            .font(Font::default())
            .scale(Scale::uniform(self.get_title_font_size(&world)?));
        let title_text = Text::new(title_fragment);

        world
            .spawn_entity()?
            .with_component(ComponentNames::Position.as_ref(), position)?
            .with_component(ComponentNames::Text.as_ref(), title_text)?;
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
