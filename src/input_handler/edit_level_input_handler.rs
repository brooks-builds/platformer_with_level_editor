use bbecs::data_types::point::Point;
use crossbeam::channel::{Receiver, Sender};
use eyre::{bail, Result};
use ggez::event::{KeyCode, MouseButton};

use crate::{
    events::{event::Event, EventManager},
    navigation::screens::NavigationScreens,
};

pub struct EditLevelInputHandler {
    level_name: Option<String>,
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

impl EditLevelInputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let level_name = None;
        let event_receiver = event_manager.subscribe(vec![
            Event::NavigatingTo(NavigationScreens::EditLevel("".to_owned())).to_string(),
            Event::MouseClicked(Point::new(0.0, 0.0)).to_string(),
        ]);
        let event_sender = event_manager.register();

        Self {
            level_name,
            event_receiver,
            event_sender,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        if let Ok(Event::NavigatingTo(NavigationScreens::EditLevel(level_name))) =
            self.event_receiver.try_recv()
        {
            self.level_name = Some(level_name);
        }
        Ok(())
    }

    pub fn handle_keyboard_input(&mut self, keycode: KeyCode) -> Result<()> {
        if keycode == KeyCode::P {
            if let Some(level_name) = &self.level_name {
                let event = Event::NavigatingTo(NavigationScreens::Play(level_name.clone()));
                self.event_sender.send(event)?;
            } else {
                bail!("We don't know the level name to switch to play mode with");
            }
        }

        Ok(())
    }

    pub fn handle_mouse_input(&mut self, mouse_button: MouseButton, x: f32, y: f32) -> Result<()> {
        if mouse_button == MouseButton::Left {
            let location = Point::new(x, y);
            let event = Event::MouseClicked(location);
            self.event_sender.send(event)?;
        }
        Ok(())
    }
}
