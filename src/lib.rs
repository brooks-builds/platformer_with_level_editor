use audio_manager::AudioManager;
use bbecs::world::{World, WorldMethods};
use events::EventManager;
use eyre::Result;
use ggez::event::{Button, EventHandler, GamepadId};
use ggez::graphics::BLACK;
use ggez::{graphics, Context};
use input_handler::InputHandler;
use loaders::LoaderManager;
use names::component_names::ComponentNames;
use names::resource_names::ResourceNames;
use navigation::Navigation;
use system_manager::SystemManager;

mod audio_manager;
mod command;
mod events;
mod helpers;
mod input_handler;
mod loaders;
mod names;
mod navigation;
mod system_manager;

pub struct MainState {
    world: World,
    event_manager: EventManager,
    loader_manager: LoaderManager,
    system_manager: SystemManager,
    input_handler: InputHandler,
    audio_manager: AudioManager,
    navigation: Navigation,
}

impl MainState {
    pub fn new(context: &mut Context) -> Result<Self> {
        let world = World::new();
        let mut event_manager = EventManager::new();
        let loader_manager = LoaderManager::new(&mut event_manager);
        let system_manager = SystemManager::new(&mut event_manager);
        let input_handler = InputHandler::new(&mut event_manager);
        let audio_manager = AudioManager::new(context, &mut event_manager)?;
        let navigation = Navigation::new(&mut event_manager);

        Ok(Self {
            world,
            event_manager,
            loader_manager,
            system_manager,
            input_handler,
            audio_manager,
            navigation,
        })
    }

    pub fn setup(&mut self, context: &mut Context, game_name: &str) -> Result<()> {
        self.world
            .add_resource(ResourceNames::GameName.to_string(), game_name.to_owned());
        self.world
            .add_resource(ResourceNames::TitleFontSize.to_string(), 72.0_f32);
        self.world
            .add_resource(ResourceNames::FontSize.to_string(), 24.0_f32);

        self.world.register(ComponentNames::Text.to_string())?;
        self.world.register(ComponentNames::Position.to_string())?;
        self.world.register(ComponentNames::Selected.to_string())?;
        self.world
            .register(ComponentNames::Selectable.to_string())?;
        self.world
            .register(ComponentNames::TextFragment.to_string())?;
        self.world
            .register(ComponentNames::NavigateTo.to_string())?;

        self.loader_manager.setup(&mut self.world, context)?;
        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        self.event_manager.process().unwrap();
        self.loader_manager
            .update(&mut self.world, context)
            .unwrap();
        self.input_handler.update().unwrap();
        self.system_manager.update(&self.world, context).unwrap();
        self.audio_manager.run().unwrap();
        self.navigation.update().unwrap();
        self.world.update().unwrap();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);
        self.system_manager.display(&self.world, context).unwrap();

        graphics::present(context)
    }

    fn gamepad_button_down_event(
        &mut self,
        _context: &mut Context,
        button: Button,
        _id: GamepadId,
    ) {
        self.input_handler
            .handle_controller_input(button, &self.world, &mut self.navigation)
            .unwrap();
    }
}
