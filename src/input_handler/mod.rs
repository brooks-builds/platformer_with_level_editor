use bbecs::world::World;
use crossbeam::channel::Sender;
use eyre::Result;
use ggez::event::Button;
use ggez::event::KeyCode;

use crate::command::Command;
use crate::events::event::Event;
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
    event_sender: Sender<Event>,
}

impl InputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_sender = event_manager.register();
        let title_screen = TitleScreenInputHandler::new(event_sender.clone());
        let level_select = LevelSelectInputHandler::new(event_sender.clone());
        let settings = SettingsInputHandler::new(event_sender.clone());

        Self {
            title_screen,
            level_select,
            settings,
            event_sender,
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
            NavigationScreens::Play(_) => match button {
                Button::DPadLeft => {
                    let command = Command::StartMovingLeft;
                    self.event_sender.send(Event::Command(command))?;
                }
                Button::DPadRight => {
                    let command = Command::StartMovingRight;
                    self.event_sender.send(Event::Command(command))?;
                }
                _ => {}
            },
            NavigationScreens::EditLevel => {}
        }
        Ok(())
    }

    pub fn handle_stop_controller_input(
        &mut self,
        button: Button,
        _world: &World,
        navigation: &mut Navigation,
    ) -> Result<()> {
        match navigation.get_current_screen() {
            NavigationScreens::Title => {}
            NavigationScreens::LevelSelect => {}
            NavigationScreens::Settings => {}
            NavigationScreens::Credits | NavigationScreens::Unknown => {}
            NavigationScreens::Play(_) => match button {
                Button::DPadLeft => {
                    let command = Command::StopMovingLeft;
                    self.event_sender.send(Event::Command(command))?;
                }
                Button::DPadRight => {
                    let command = Command::StopMovingRight;
                    self.event_sender.send(Event::Command(command))?;
                }
                _ => {}
            },
            NavigationScreens::EditLevel => todo!(),
        }
        Ok(())
    }

    pub fn handle_keyboard_input(
        &mut self,
        keycode: KeyCode,
        navigation: &mut Navigation,
    ) -> Result<()> {
        match navigation.get_current_screen() {
            NavigationScreens::Title => {}
            NavigationScreens::LevelSelect => {}
            NavigationScreens::Settings => {}
            NavigationScreens::Credits => {}
            NavigationScreens::Unknown => {}
            NavigationScreens::Play(_level_name) => {
                let event = Event::NavigatingTo(NavigationScreens::EditLevel);
                self.event_sender.send(event)?;
            }
            NavigationScreens::EditLevel => {
                let event = Event::NavigatingTo(NavigationScreens::Play("".to_string()));
                if let KeyCode::P = keycode {
                    self.event_sender.send(event)?;
                }
            }
        }
        Ok(())
    }
}
