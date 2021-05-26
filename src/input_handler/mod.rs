use bbecs::world::World;
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::event::Button;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::states::navigation::Navigation;

use self::level_select::LevelSelectInputHandler;
use self::settings::SettingsInputHandler;
use self::title_screen::TitleScreenInputHandler;

mod level_select;
mod settings;
mod title_screen;

trait HandleInput {
    fn handle_controller_input(&mut self, world: &World, button: Button) -> Result<()>;
}

pub struct InputHandler {
    event_receiver: Receiver<Event>,
    current_navigation: Navigation,
    title_screen: TitleScreenInputHandler,
    level_select: LevelSelectInputHandler,
    settings: SettingsInputHandler,
}

impl InputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager.subscribe(vec![
            Event::NavigatingTo(Navigation::TitleScreen).to_string(),
            Event::NavigatingTo(Navigation::SelectLevel).to_string(),
        ]);
        dbg!(Event::NavigatingTo(Navigation::SelectLevel).to_string());
        let current_navigation = Navigation::TitleScreen;
        let event_sender = event_manager.register();
        let title_screen = TitleScreenInputHandler::new(event_sender.clone());
        let level_select = LevelSelectInputHandler::new(event_sender.clone());
        let settings = SettingsInputHandler::new(event_sender);

        Self {
            event_receiver,
            current_navigation,
            title_screen,
            level_select,
            settings,
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
        dbg!(&self.current_navigation);
        match self.current_navigation {
            Navigation::TitleScreen => self.title_screen.handle_controller_input(world, button)?,
            Navigation::SelectLevel => self.level_select.handle_controller_input(world, button)?,
            Navigation::Credits => {}
            Navigation::Settings => self.settings.handle_controller_input(world, button)?,
        }
        Ok(())
    }
}
