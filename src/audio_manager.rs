use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::audio::{SoundSource, Source};
use ggez::Context;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::navigation::screens::NavigationScreens;

pub struct AudioManager {
    event_receiver: Receiver<Event>,
    menu_navigate: Source,
    menu_select: Source,
    won: Source,
}

impl AudioManager {
    pub fn new(context: &mut Context, event_manager: &mut EventManager) -> Result<Self> {
        let mut menu_select = Source::new(context, "/menu_select.mp3")?;
        menu_select.set_volume(0.08);
        let event_receiver = event_manager.subscribe(vec![
            Event::ChangeMenuItem.to_string(),
            Event::NavigatingTo(NavigationScreens::LevelSelect).to_string(),
            Event::Won.to_string(),
        ]);
        let mut menu_navigate = Source::new(context, "/menu_navigate.mp3")?;
        menu_navigate.set_volume(0.6);
        let mut won = Source::new(context, "/win_game_sound.mp3")?;
        won.set_volume(0.08);
        won.set_repeat(false);

        Ok(Self {
            event_receiver,
            menu_navigate,
            menu_select,
            won,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::NavigatingTo(_) => self.menu_navigate.play()?,
                Event::Command(_) => {}
                Event::ChangeMenuItem => self.menu_select.play()?,
                Event::Won => self.won.play()?,
            }
        }
        Ok(())
    }
}
