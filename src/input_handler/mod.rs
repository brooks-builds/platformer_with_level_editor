use crossbeam::channel::{Receiver, Sender};
use eyre::Result;
use ggez::event::Button;

use crate::command::Command;
use crate::events::event::Event;
use crate::events::EventManager;
use crate::states::navigation::Navigation;

pub struct InputHandler {
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
    current_navigation: Navigation,
}

impl InputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(Navigation::TitleScreen).to_string()
            ]);
        let current_navigation = Navigation::_Settings;
        let event_sender = event_manager.register();

        Self {
            event_receiver,
            event_sender,
            current_navigation,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::NavigatingTo(new_navigation) => self.current_navigation = new_navigation,
                Event::Command(_) => {}
                Event::ChangeMenuItem => {}
            }
        }
        Ok(())
    }

    pub fn handle_controller_input(&mut self, button: Button) -> Result<()> {
        match self.current_navigation {
            Navigation::TitleScreen => match button {
                Button::Start => self.event_sender.send(Event::Command(Command::Select))?,
                Button::DPadUp => self.event_sender.send(Event::Command(Command::SelectUp))?,
                Button::DPadDown => self
                    .event_sender
                    .send(Event::Command(Command::SelectDown))?,
                _ => {}
            },
            Navigation::_Settings => {}
        }
        Ok(())
    }
}
