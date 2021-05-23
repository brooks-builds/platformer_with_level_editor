use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::{Text, TextFragment};
use ggez::{graphics, Context};

use crate::names::component_names::ComponentNames;

#[derive(Default)]
pub struct UpdateTextSystem;

impl UpdateTextSystem {
    pub fn run(&self, world: &World, context: &mut Context) -> Result<()> {
        let query;
        let (text_fragment_components, selected_components, position_components, text_components) = query!(
            world,
            query,
            ComponentNames::TextFragment.as_ref(),
            ComponentNames::Selected.as_ref(),
            ComponentNames::Position.as_ref(),
            ComponentNames::Text.as_ref()
        );
        for (index, text_fragment_component) in text_fragment_components.iter().enumerate() {
            let wrapped_text_fragment: &DataWrapper<TextFragment> =
                text_fragment_component.cast()?;
            let wrapped_selected: &DataWrapper<bool> = selected_components[index].cast()?;
            let wrapped_position: &DataWrapper<Point> = position_components[index].cast()?;
            let wrapped_text: &DataWrapper<Text> = text_components[index].cast()?;

            if !*wrapped_selected.borrow() && wrapped_text_fragment.borrow().text.starts_with('<') {
                self.remove_selected(wrapped_text_fragment);
                self.update_text(wrapped_text_fragment, wrapped_text);
                self.update_position(wrapped_position, context, wrapped_text);
            } else if *wrapped_selected.borrow()
                && !wrapped_text_fragment.borrow().text.starts_with('<')
            {
                self.add_selected(wrapped_text_fragment);
                self.update_text(wrapped_text_fragment, wrapped_text);
                self.update_position(wrapped_position, context, wrapped_text);
            }
        }
        Ok(())
    }

    fn remove_selected(&self, wrapped_text_fragment: &DataWrapper<TextFragment>) {
        let string = &mut wrapped_text_fragment.borrow_mut().text;
        string.remove(0);
        string.pop();
    }

    fn add_selected(&self, wrapped_text_fragment: &DataWrapper<TextFragment>) {
        let string = &mut wrapped_text_fragment.borrow_mut().text;
        string.insert(0, '<');
        string.push('>');
    }

    fn update_text(
        &self,
        wrapped_text_fragment: &DataWrapper<TextFragment>,
        wrapped_text: &DataWrapper<Text>,
    ) {
        let mut text = wrapped_text.borrow_mut();
        *text = Text::new(wrapped_text_fragment.borrow().clone());
    }

    fn update_position(
        &self,
        wrapped_position: &DataWrapper<Point>,
        context: &mut Context,
        wrapped_text: &DataWrapper<Text>,
    ) {
        let mut position = wrapped_position.borrow_mut();
        let screen_coordinates = graphics::screen_coordinates(context);
        position.x = screen_coordinates.w / 2.0 - wrapped_text.borrow().width(context) as f32 / 2.0;
    }
}
