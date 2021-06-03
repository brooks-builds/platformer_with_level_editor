use crossbeam::channel::{Receiver, Sender};
use eyre::Result;
use ggez::event::Button;

use crate::{
    command::Command,
    events::{event::Event, EventManager},
    navigation::screens::NavigationScreens,
};

pub struct PlayInputHandler {
    won_state: bool,
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

impl PlayInputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let won_state = false;
        let event_receiver = event_manager.subscribe(vec![Event::Won.to_string()]);
        let event_sender = event_manager.register();

        Self {
            won_state,
            event_receiver,
            event_sender,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        if let Ok(Event::Won) = self.event_receiver.try_recv() {
            self.won_state = true;
        }

        Ok(())
    }

    pub fn handle_controller_input(&mut self, button: Button) -> Result<()> {
        match button {
            Button::DPadLeft => {
                let command = Command::StartMovingLeft;
                self.event_sender.send(Event::Command(command))?;
            }
            Button::DPadRight => {
                let command = Command::StartMovingRight;
                self.event_sender.send(Event::Command(command))?;
            }
            Button::South => {
                let command = Command::Jump;
                self.event_sender.send(Event::Command(command))?;
            }
            Button::Start => {
                if self.won_state {
                    self.event_sender
                        .send(Event::NavigatingTo(NavigationScreens::LevelSelect))?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
