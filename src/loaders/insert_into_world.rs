use bbecs::{
    data_types::point::Point,
    world::{World, WorldMethods},
};
use eyre::Result;
use ggez::graphics::{Text, TextFragment};

use crate::names::component_names::ComponentNames;

#[derive(Default)]
pub struct InsertIntoWorld {
    navigate_to: Option<String>,
    position: Option<Point>,
    selectable: Option<bool>,
    selected: Option<bool>,
    text: Option<Text>,
    text_fragment: Option<TextFragment>,
}

impl InsertIntoWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_navigate_to(mut self, navigate_to: String) -> Self {
        self.navigate_to = Some(navigate_to);
        self
    }

    pub fn set_position(mut self, position: Point) -> Self {
        self.position = Some(position);
        self
    }

    pub fn set_selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    pub fn set_selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn set_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    pub fn set_text_fragment(mut self, text_fragment: TextFragment) -> Self {
        self.text_fragment = Some(text_fragment);
        self
    }

    pub fn insert(self, world: &mut World) -> Result<()> {
        world.spawn_entity()?;

        if let Some(navigate_to) = self.navigate_to {
            world.with_component(ComponentNames::NavigateTo.as_ref(), navigate_to)?;
        }

        if let Some(position) = self.position {
            world.with_component(ComponentNames::Position.as_ref(), position)?;
        }

        if let Some(selectable) = self.selectable {
            world.with_component(ComponentNames::Selectable.as_ref(), selectable)?;
        }

        if let Some(selected) = self.selected {
            world.with_component(ComponentNames::Selected.as_ref(), selected)?;
        }

        if let Some(text) = self.text {
            world.with_component(ComponentNames::Text.as_ref(), text)?;
        }

        if let Some(text_fragment) = self.text_fragment {
            world.with_component(ComponentNames::TextFragment.as_ref(), text_fragment)?;
        }

        Ok(())
    }
}
