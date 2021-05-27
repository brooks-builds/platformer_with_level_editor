use crossbeam::channel::Receiver;
use eyre::Result;

use crate::events::{event::Event, EventManager};

use self::screens::NavigationScreens;

pub mod screens;

pub struct Navigation {
    history: Vec<NavigationScreens>,
    event_receiver: Receiver<Event>,
}

impl Navigation {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let history = vec![NavigationScreens::Title];
        let events_to_subscribe_to =
            vec![Event::NavigatingTo(NavigationScreens::Title).to_string()];
        let event_receiver = event_manager.subscribe(events_to_subscribe_to);
        Self {
            history,
            event_receiver,
        }
    }

    pub fn get_current_screen(&self) -> NavigationScreens {
        *self.history.last().unwrap_or(&NavigationScreens::Unknown)
    }

    pub fn pop(&mut self) -> NavigationScreens {
        self.history.pop().unwrap_or(NavigationScreens::Unknown)
    }

    pub fn create_navigation_screen_from_string(&self, navigations_str: &str) -> NavigationScreens {
        match navigations_str {
            "Title" => NavigationScreens::Title,
            "LevelSelect" => NavigationScreens::LevelSelect,
            "Settings" => NavigationScreens::Settings,
            "Credits" => NavigationScreens::Credits,
            "Play" => NavigationScreens::Play,
            _ => NavigationScreens::Unknown,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Event::NavigatingTo(navigation_screen) = event {
                self.history.push(navigation_screen);
            }
        }
        Ok(())
    }
}
