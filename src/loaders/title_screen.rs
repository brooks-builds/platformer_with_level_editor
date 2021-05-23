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
        self.create_credits(world, screen_coordinates, context)?;

        Ok(())
    }

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
        let title_text = Text::new(title_text_fragment.clone());
        let title_position = Point::new(
            screen_coordinates.w / 2.0 - title_text.width(context) as f32 / 2.0,
            title_text.height(context) as f32 * 2.0,
        );

        self.insert_into_world(
            world,
            title_text,
            title_text_fragment,
            title_position,
            false,
            false,
        )?;

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

        self.insert_into_world(
            world,
            play_text,
            play_text_fragment,
            play_position,
            true,
            true,
        )?;

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

        self.insert_into_world(
            world,
            settings_text,
            settings_text_fragment,
            settings_position,
            false,
            true,
        )?;

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

        self.insert_into_world(
            world,
            credits_text,
            credits_text_fragment,
            credits_position,
            false,
            true,
        )?;

        Ok(())
    }

    fn insert_into_world(
        &self,
        world: &mut World,
        text: Text,
        text_fragment: TextFragment,
        position: Point,
        selected: bool,
        selectable: bool,
    ) -> Result<()> {
        world
            .spawn_entity()?
            .with_component(ComponentNames::Text.as_ref(), text)?
            .with_component(ComponentNames::Position.as_ref(), position)?
            .with_component(ComponentNames::Selected.as_ref(), selected)?
            .with_component(ComponentNames::TextFragment.as_ref(), text_fragment)?;
        if selectable {
            world.with_component(ComponentNames::Selectable.as_ref(), selectable)?;
        }
        Ok(())
    }
}
