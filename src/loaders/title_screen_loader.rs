use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{self, Rect, Scale, Text, TextFragment, WHITE};
use ggez::Context;

use crate::helpers::get_resource;
use crate::level_manager::LevelManager;
use crate::names::resource_names::ResourceNames;
use crate::navigation::screens::NavigationScreens;

use super::insert_into_world::InsertIntoWorld;

pub struct TitleScreenLoader;

impl TitleScreenLoader {
    fn create_text(&self, name: &str, font_size: f32) -> TextFragment {
        TextFragment::new(name)
            .color(WHITE)
            .scale(Scale::uniform(font_size))
    }

    fn create_title(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let title_text_fragment = self.create_text(
            &get_resource::get_string(&world, ResourceNames::GameName.as_ref()),
            get_resource::get_f32(&world, ResourceNames::TitleFontSize.as_ref()),
        );
        let title_text = Text::new(title_text_fragment);
        let title_position = Point::new(
            screen_coordinates.w / 2.0 - title_text.width(context) as f32 / 2.0,
            title_text.height(context) as f32 * 2.0,
        );

        InsertIntoWorld::new()
            .set_position(title_position)
            .set_text(title_text)
            .insert(world)?;

        Ok(())
    }

    fn create_play(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let play_text_fragment = self.create_text(
            "<Play>",
            get_resource::get_f32(&world, ResourceNames::FontSize.as_ref()),
        );
        let play_text = Text::new(play_text_fragment.clone());
        let play_position = Point::new(
            screen_coordinates.w / 2.0 - play_text.width(context) as f32 / 2.0,
            screen_coordinates.h - screen_coordinates.h * 0.25 - play_text.height(context) as f32,
        );

        InsertIntoWorld::new()
            .set_navigate_to(NavigationScreens::LevelSelect.to_string())
            .set_position(play_position)
            .set_selectable(true)
            .set_selected(true)
            .set_text(play_text)
            .set_text_fragment(play_text_fragment)
            .insert(world)?;

        Ok(())
    }

    fn create_settings(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let settings_text_fragment = self.create_text(
            "Settings",
            get_resource::get_f32(&world, ResourceNames::FontSize.as_ref()),
        );
        let settings_text = Text::new(settings_text_fragment.clone());
        let settings_position = Point::new(
            screen_coordinates.w / 2.0 - settings_text.width(context) as f32 / 2.0,
            screen_coordinates.h - screen_coordinates.h * 0.25
                + settings_text.height(context) as f32,
        );

        InsertIntoWorld::new()
            .set_navigate_to(NavigationScreens::Settings.to_string())
            .set_position(settings_position)
            .set_selectable(true)
            .set_selected(false)
            .set_text(settings_text)
            .set_text_fragment(settings_text_fragment)
            .insert(world)?;

        Ok(())
    }

    fn create_credits(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let credits_text_fragment = self.create_text(
            "Credits",
            get_resource::get_f32(&world, ResourceNames::FontSize.as_ref()),
        );
        let credits_text = Text::new(credits_text_fragment.clone());
        let credits_position = Point::new(
            screen_coordinates.w / 2.0 - credits_text.width(context) as f32 / 2.0,
            screen_coordinates.h - screen_coordinates.h * 0.25
                + credits_text.height(context) as f32 * 3.0,
        );

        InsertIntoWorld::new()
            .set_navigate_to(NavigationScreens::Credits.to_string())
            .set_position(credits_position)
            .set_selectable(true)
            .set_selected(false)
            .set_text(credits_text)
            .set_text_fragment(credits_text_fragment)
            .insert(world)?;

        Ok(())
    }

    pub fn load(
        &mut self,
        world: &mut World,
        context: &mut Context,
        _level_manager: &LevelManager,
    ) -> Result<()> {
        let screen_coordinates = graphics::screen_coordinates(context);
        self.create_title(world, screen_coordinates, context)?;
        self.create_play(world, screen_coordinates, context)?;
        self.create_settings(world, screen_coordinates, context)?;
        self.create_credits(world, screen_coordinates, context)?;

        Ok(())
    }
}
