use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::audio::{SoundSource, Source};
use ggez::Context;

use crate::events::event::Event;
use crate::events::EventManager;

pub struct AudioManager {
    menu_select: Source,
    event_receiver: Receiver<Event>,
}

impl AudioManager {
    pub fn new(context: &mut Context, event_manager: &mut EventManager) -> Result<Self> {
        let menu_select = Source::new(context, "/menu_select.mp3")?;
        let event_receiver = event_manager.subscribe(vec![Event::ChangeMenuItem.to_string()]);

        Ok(Self {
            menu_select,
            event_receiver,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::NavigatingTo(_) => {}
                Event::Command(_) => {}
                Event::ChangeMenuItem => self.menu_select.play()?,
            }
        }
        Ok(())
    }
}
