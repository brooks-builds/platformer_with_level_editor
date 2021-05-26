use bbecs::components::CastComponents;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use crossbeam::channel::Sender;
use eyre::Result;
use ggez::event::Button;

use crate::command::Command;
use crate::events::event::Event;
use crate::names::component_names::ComponentNames;
use crate::navigation::Navigation;

pub struct TitleScreenInputHandler {
    event_sender: Sender<Event>,
}

impl TitleScreenInputHandler {
    pub fn new(event_sender: Sender<Event>) -> Self {
        Self { event_sender }
    }

    pub fn handle_controller_input(
        &mut self,
        world: &World,
        button: Button,
        navigation: &Navigation,
    ) -> Result<()> {
        match button {
            Button::Start => self.handle_select(world, navigation)?,
            Button::DPadUp => self.event_sender.send(Event::Command(Command::SelectUp))?,
            Button::DPadDown => self
                .event_sender
                .send(Event::Command(Command::SelectDown))?,
            Button::South => self.handle_select(world, navigation)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_select(&mut self, world: &World, navigation: &Navigation) -> Result<()> {
        let query;
        let (selected_components, navigate_to_components) = query!(
            world,
            query,
            ComponentNames::Selected.as_ref(),
            ComponentNames::NavigateTo.as_ref()
        );

        for (index, selected_component) in selected_components.iter().enumerate() {
            let wrapped_selected: &DataWrapper<bool> = selected_component.cast()?;
            if !*wrapped_selected.borrow() {
                continue;
            }
            let wrapped_navigate_string: &DataWrapper<String> =
                navigate_to_components[index].cast()?;
            let navigate_to =
                navigation.create_navigation_screen_from_string(&*wrapped_navigate_string.borrow());
            self.event_sender.send(Event::NavigatingTo(navigate_to))?;
        }
        Ok(())
    }
}
