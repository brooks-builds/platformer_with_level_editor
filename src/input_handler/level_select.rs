use bbecs::{components::CastComponents, query, world::DataWrapper};
use crossbeam::channel::Sender;
use eyre::{bail, Result};

use crate::{
    command::Command,
    events::event::Event,
    names::component_names::ComponentNames,
    navigation::{screens::NavigationScreens, Navigation},
};

use super::HandleInput;

pub struct LevelSelectInputHandler {
    event_sender: Sender<Event>,
}

impl LevelSelectInputHandler {
    pub fn new(event_sender: Sender<Event>) -> Self {
        Self { event_sender }
    }

    fn send_back_event(&mut self, navigation: &mut Navigation) -> Result<()> {
        navigation.pop();
        let event = Event::NavigatingTo(navigation.get_current_screen());
        self.event_sender.send(event)?;
        Ok(())
    }

    fn navigate_to_selected_level(&mut self, world: &bbecs::world::World) -> Result<()> {
        let level_name = self.query_for_selected_level_name(world)?;
        let event = Event::NavigatingTo(NavigationScreens::Play(level_name));
        self.event_sender.send(event)?;
        Ok(())
    }

    fn query_for_selected_level_name(&self, world: &bbecs::world::World) -> Result<String> {
        let query;
        let (names, selected_components) = query!(
            world,
            query,
            ComponentNames::Name.as_ref(),
            ComponentNames::Selected.as_ref()
        );

        for (index, selected_component) in selected_components.iter().enumerate() {
            let wrapped_selected: &DataWrapper<bool> = selected_component.cast()?;
            if *wrapped_selected.borrow() {
                let wrapped_name: &DataWrapper<String> = names[index].cast()?;
                return Ok(wrapped_name.borrow().clone());
            }
        }
        bail!("Could not find a selected level");
    }
}

impl HandleInput for LevelSelectInputHandler {
    fn handle_controller_input(
        &mut self,
        world: &bbecs::world::World,
        button: ggez::event::Button,
        navigation: &mut Navigation,
    ) -> eyre::Result<()> {
        match button {
            ggez::event::Button::South => self.navigate_to_selected_level(world)?,
            ggez::event::Button::East => self.send_back_event(navigation)?,
            ggez::event::Button::North => {}
            ggez::event::Button::West => {}
            ggez::event::Button::C => {}
            ggez::event::Button::Z => {}
            ggez::event::Button::LeftTrigger => {}
            ggez::event::Button::LeftTrigger2 => {}
            ggez::event::Button::RightTrigger => {}
            ggez::event::Button::RightTrigger2 => {}
            ggez::event::Button::Select => self.send_back_event(navigation)?,
            ggez::event::Button::Start => self.navigate_to_selected_level(world)?,
            ggez::event::Button::Mode => {}
            ggez::event::Button::LeftThumb => {}
            ggez::event::Button::RightThumb => {}
            ggez::event::Button::DPadUp => {
                self.event_sender.send(Event::Command(Command::SelectUp))?
            }
            ggez::event::Button::DPadDown => self
                .event_sender
                .send(Event::Command(Command::SelectDown))?,
            ggez::event::Button::DPadLeft => {}
            ggez::event::Button::DPadRight => {}
            ggez::event::Button::Unknown => {}
        }
        Ok(())
    }
}
