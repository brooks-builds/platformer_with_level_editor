use bbecs::world::World;
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::Context;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::states::navigation::Navigation;

use self::title_screen::TitleScreenLoader;

mod title_screen;

pub struct LoaderManager {
    event_receiver: Receiver<Event>,
    title_screen: TitleScreenLoader,
}

impl LoaderManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(Navigation::TitleScreen).to_string()
            ]);
        let title_screen = TitleScreenLoader;

        Self {
            event_receiver,
            title_screen,
        }
    }

    pub fn update(&mut self, world: &mut World, context: &mut Context) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Event::NavigatingTo(target) = event {
                match target {
                    Navigation::TitleScreen => self.title_screen.load(world, context)?,
                    Navigation::Play => {}
                    Navigation::Credits => {}
                    Navigation::Settings => {}
                }
            }
        }
        Ok(())
    }
}
