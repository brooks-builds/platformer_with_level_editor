use bbecs::world::World;
use eyre::Result;
use ggez::event::Button;

use crate::events::EventManager;
use crate::navigation::screens::NavigationScreens;
use crate::navigation::Navigation;

use self::level_select::LevelSelectInputHandler;
use self::settings::SettingsInputHandler;
use self::title_screen::TitleScreenInputHandler;

mod level_select;
mod settings;
mod title_screen;

trait HandleInput {
    fn handle_controller_input(
        &mut self,
        world: &World,
        button: Button,
        navigation: &mut Navigation,
    ) -> Result<()>;
}

pub struct InputHandler {
    title_screen: TitleScreenInputHandler,
    level_select: LevelSelectInputHandler,
    settings: SettingsInputHandler,
}

impl InputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_sender = event_manager.register();
        let title_screen = TitleScreenInputHandler::new(event_sender.clone());
        let level_select = LevelSelectInputHandler::new(event_sender.clone());
        let settings = SettingsInputHandler::new(event_sender);

        Self {
            title_screen,
            level_select,
            settings,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn handle_controller_input(
        &mut self,
        button: Button,
        world: &World,
        navigation: &mut Navigation,
    ) -> Result<()> {
        let current_screen = navigation.get_current_screen();
        match current_screen {
            NavigationScreens::Title => {
                self.title_screen
                    .handle_controller_input(world, button, &navigation)?
            }
            NavigationScreens::LevelSelect => self
                .level_select
                .handle_controller_input(world, button, navigation)?,
            NavigationScreens::Settings => self
                .settings
                .handle_controller_input(world, button, navigation)?,
            NavigationScreens::Credits => {}
            NavigationScreens::Unknown => {}
        }
        Ok(())
    }
}
