use crossbeam::channel::Sender;
use eyre::Result;

use crate::{events::event::Event, states::navigation::Navigation};

use super::HandleInput;

pub struct SettingsInputHandler {
    event_sender: Sender<Event>,
}

impl SettingsInputHandler {
    pub fn new(event_sender: Sender<Event>) -> Self {
        Self { event_sender }
    }

    fn send_back_event(&mut self) -> Result<()> {
        let event = Event::NavigatingTo(Navigation::TitleScreen);
        self.event_sender.send(event)?;
        Ok(())
    }
}

impl HandleInput for SettingsInputHandler {
    fn handle_controller_input(
        &mut self,
        world: &bbecs::world::World,
        button: ggez::event::Button,
    ) -> eyre::Result<()> {
        match button {
            ggez::event::Button::South => {}
            ggez::event::Button::East => self.send_back_event()?,
            ggez::event::Button::North => {}
            ggez::event::Button::West => {}
            ggez::event::Button::C => {}
            ggez::event::Button::Z => {}
            ggez::event::Button::LeftTrigger => {}
            ggez::event::Button::LeftTrigger2 => {}
            ggez::event::Button::RightTrigger => {}
            ggez::event::Button::RightTrigger2 => {}
            ggez::event::Button::Select => self.send_back_event()?,
            ggez::event::Button::Start => {}
            ggez::event::Button::Mode => {}
            ggez::event::Button::LeftThumb => {}
            ggez::event::Button::RightThumb => {}
            ggez::event::Button::DPadUp => {}
            ggez::event::Button::DPadDown => {}
            ggez::event::Button::DPadLeft => {}
            ggez::event::Button::DPadRight => {}
            ggez::event::Button::Unknown => {}
        }
        Ok(())
    }
}
