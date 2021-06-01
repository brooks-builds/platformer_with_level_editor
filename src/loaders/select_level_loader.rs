use bbecs::data_types::point::Point;
use bbecs::get_resource;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{Font, Scale, Text, TextFragment, WHITE};
use ggez::Context;

use crate::helpers::get_resource::get_f32;
use crate::level_manager::LevelManager;
use crate::names::resource_names::ResourceNames;

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

    fn create_list_of_levels(&self, world: &mut World, level_manager: &LevelManager) -> Result<()> {
        let levels = level_manager.get_all_levels();
        let font_size = get_f32(&world, ResourceNames::FontSize.as_ref());

        for (index, level) in levels.iter().enumerate() {
            let text_fragment =
                TextFragment::new(level.name.clone()).scale(Scale::uniform(font_size));
            let text = Text::new(text_fragment.clone());
            let position = Point::new(10.0, 100.0 + index as f32 * font_size);
            let selected = index == 0;

            InsertIntoWorld::new()
                .set_text(text)
                .set_position(position)
                .set_selectable(true)
                .set_selected(selected)
                .set_text_fragment(text_fragment)
                .insert(world)?;
        }

        Ok(())
    }
}

impl Loader for SelectLevelLoader {
    fn load(
        &mut self,
        world: &mut World,
        context: &mut Context,
        level_manager: &LevelManager,
    ) -> Result<()> {
        self.create_title(world, context)?;
        self.create_list_of_levels(world, level_manager)?;
        Ok(())
    }
}
