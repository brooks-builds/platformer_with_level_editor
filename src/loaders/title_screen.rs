use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::{self, Rect, Scale, Text, TextFragment, WHITE};
use ggez::Context;

use crate::helpers::get_resource;
use crate::names::component_names::ComponentNames;
use crate::names::resource_names::ResourceNames;

pub struct TitleScreenLoader;

impl TitleScreenLoader {
    pub fn load(&mut self, world: &mut World, context: &mut Context) -> Result<()> {
        let screen_coordinates = graphics::screen_coordinates(context);
        self.create_title(world, screen_coordinates, context)?;
        self.create_play(world, screen_coordinates, context)?;
        self.create_settings(world, screen_coordinates, context)?;

        Ok(())
    }

    fn create_text(&self, name: &str, font_size: f32) -> Text {
        Text::new(
            TextFragment::new(name)
                .color(WHITE)
                .scale(Scale::uniform(font_size)),
        )
    }

    fn create_title(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let title_text = self.create_text(
            &get_resource::get_string(&world, ResourceNames::GameName.as_ref()),
            get_resource::get_f32(&world, ResourceNames::TitleFontSize.as_ref()),
        );
        let title_position = Point::new(
            screen_coordinates.w / 2.0 - title_text.width(context) as f32 / 2.0,
            title_text.height(context) as f32 * 2.0,
        );

        self.insert_into_world(world, title_text, title_position)?;

        Ok(())
    }

    fn create_play(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let play_text = self.create_text(
            "Play",
            get_resource::get_f32(&world, ResourceNames::FontSize.as_ref()),
        );
        let play_position = Point::new(
            screen_coordinates.w / 2.0 - play_text.width(context) as f32 / 2.0,
            screen_coordinates.h - screen_coordinates.h * 0.25 - play_text.height(context) as f32,
        );

        self.insert_into_world(world, play_text, play_position)?;

        Ok(())
    }

    fn create_settings(
        &self,
        world: &mut World,
        screen_coordinates: Rect,
        context: &mut Context,
    ) -> Result<()> {
        let settings_text = self.create_text(
            "Settings",
            get_resource::get_f32(&world, ResourceNames::FontSize.as_ref()),
        );

        let settings_position = Point::new(
            screen_coordinates.w / 2.0 - settings_text.width(context) as f32 / 2.0,
            screen_coordinates.h - screen_coordinates.h * 0.25
                + settings_text.height(context) as f32,
        );

        self.insert_into_world(world, settings_text, settings_position)?;

        Ok(())
    }

    fn insert_into_world(&self, world: &mut World, text: Text, position: Point) -> Result<()> {
        world
            .spawn_entity()?
            .with_component(ComponentNames::Text.as_ref(), text)?
            .with_component(ComponentNames::Position.as_ref(), position)?;
        Ok(())
    }
}
