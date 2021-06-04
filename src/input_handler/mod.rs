use bbecs::world::World;
use crossbeam::channel::Sender;
use eyre::Result;
use ggez::event::Button;
use ggez::event::KeyCode;
use ggez::event::MouseButton;

use crate::command::Command;
use crate::events::event::Event;
use crate::events::EventManager;
use crate::navigation::screens::NavigationScreens;
use crate::navigation::Navigation;

use self::edit_level_input_handler::EditLevelInputHandler;
use self::level_select::LevelSelectInputHandler;
use self::play_input_handler::PlayInputHandler;
use self::settings::SettingsInputHandler;
use self::title_screen::TitleScreenInputHandler;

mod edit_level_input_handler;
mod level_select;
mod play_input_handler;
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
    play_input_handler: PlayInputHandler,
    edit_level_input_handler: EditLevelInputHandler,
}

impl InputHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_sender = event_manager.register();
        let title_screen = TitleScreenInputHandler::new(event_sender.clone());
        let level_select = LevelSelectInputHandler::new(event_sender.clone());
        let settings = SettingsInputHandler::new(event_sender.clone());
        let play_input_handler = PlayInputHandler::new(event_manager);
        let edit_level_input_handler = EditLevelInputHandler::new(event_manager);

        Self {
            title_screen,
            level_select,
            settings,
            event_sender,
            play_input_handler,
            edit_level_input_handler,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        self.play_input_handler.update()?;
        self.edit_level_input_handler.update()?;
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
            NavigationScreens::Play(_) => {
                self.play_input_handler.handle_controller_input(button)?
            }
            NavigationScreens::EditLevel(_) => {}
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
            NavigationScreens::EditLevel(_) => todo!(),
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
            NavigationScreens::Play(level_name) => {
                if let KeyCode::E = keycode {
                    let event = Event::NavigatingTo(NavigationScreens::EditLevel(level_name));
                    self.event_sender.send(event)?;
                }
            }
            NavigationScreens::EditLevel(_level_name) => self
                .edit_level_input_handler
                .handle_keyboard_input(keycode)?,
        }
        Ok(())
    }

    pub fn handle_mouse_input(
        &mut self,
        x: f32,
        y: f32,
        mouse_button: MouseButton,
        navigation: &mut Navigation,
    ) -> Result<()> {
        match navigation.get_current_screen() {
            NavigationScreens::Title => {}
            NavigationScreens::LevelSelect => {}
            NavigationScreens::Settings => {}
            NavigationScreens::Credits => {}
            NavigationScreens::Unknown => {}
            NavigationScreens::Play(_) => {}
            NavigationScreens::EditLevel(_) => {
                self.edit_level_input_handler
                    .handle_mouse_input(mouse_button, x, y)?
            }
        }

        Ok(())
    }
}
