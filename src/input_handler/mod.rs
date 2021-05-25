use bbecs::world::World;
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::event::Button;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::states::navigation::Navigation;

use self::title_screen::TitleScreenInputHandler;

mod title_screen;

pub struct InputHandler {
    event_receiver: Receiver<Event>,
    current_navigation: Navigation,
    title_screen: TitleScreenInputHandler,
}

impl InputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(Navigation::TitleScreen).to_string()
            ]);
        let current_navigation = Navigation::SelectLevel;
        let event_sender = event_manager.register();
        let title_screen = TitleScreenInputHandler::new(event_sender);

        Self {
            event_receiver,
            current_navigation,
            title_screen,
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

    pub fn handle_controller_input(&mut self, button: Button, world: &World) -> Result<()> {
        match self.current_navigation {
            Navigation::TitleScreen => self.title_screen.handle_controller_input(world, button)?,
            Navigation::SelectLevel => {}
            Navigation::Credits => {}
            Navigation::Settings => {}
        }
        Ok(())
    }
}
