use audio_manager::AudioManager;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use events::EventManager;
use eyre::Result;
use ggez::event::{Button, EventHandler, GamepadId, KeyCode, KeyMods};
use ggez::graphics::BLACK;
use ggez::{graphics, timer, Context};
use image_manager::ImageManager;
use input_handler::InputHandler;
use level_manager::LevelManager;
use loaders::LoaderManager;
use names::component_names::ComponentNames;
use names::resource_names::ResourceNames;
use navigation::Navigation;
use system_manager::SystemManager;

mod audio_manager;
mod command;
mod events;
mod helpers;
mod image_manager;
mod input_handler;
mod level_manager;
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
    level_manager: LevelManager,
    image_manager: ImageManager,
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
        let level_manager = LevelManager::new();
        let image_manager = ImageManager::new(context)?;

        Ok(Self {
            world,
            event_manager,
            loader_manager,
            system_manager,
            input_handler,
            audio_manager,
            navigation,
            level_manager,
            image_manager,
        })
    }

    pub fn setup(&mut self, context: &mut Context, game_name: &str) -> Result<()> {
        self.world
            .add_resource(ResourceNames::GameName.to_string(), game_name.to_owned());
        self.world
            .add_resource(ResourceNames::TitleFontSize.to_string(), 72.0_f32);
        self.world
            .add_resource(ResourceNames::FontSize.to_string(), 24.0_f32);
        self.world.add_resource(
            ResourceNames::UnitSize.to_string(),
            Point::new(150.0, 150.0),
        );
        self.world
            .add_resource(ResourceNames::Gravity.to_string(), 0.01_f32);
        self.world
            .add_resource(ResourceNames::PlayerMoveSpeed.to_string(), 0.1_f32);

        self.world.register(ComponentNames::Text.to_string())?;
        self.world.register(ComponentNames::Position.to_string())?;
        self.world.register(ComponentNames::Selected.to_string())?;
        self.world
            .register(ComponentNames::Selectable.to_string())?;
        self.world
            .register(ComponentNames::TextFragment.to_string())?;
        self.world
            .register(ComponentNames::NavigateTo.to_string())?;
        self.world.register(ComponentNames::Velocity.to_string())?;
        self.world
            .register(ComponentNames::Acceleration.to_string())?;
        self.world
            .register(ComponentNames::AffectedByGravity.to_string())?;
        self.world.register(ComponentNames::Camera.to_string())?;
        self.world.register(ComponentNames::Width.to_string())?;
        self.world.register(ComponentNames::Height.to_string())?;
        self.world.register(ComponentNames::Platform.to_string())?;
        self.world.register(ComponentNames::ImageName.to_string())?;
        self.world.register(ComponentNames::Player.to_string())?;

        self.loader_manager
            .setup(&mut self.world, context, &self.level_manager)?;
        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        while timer::check_update_time(context, 60) {
            self.event_manager.process().unwrap();
            self.loader_manager
                .update(&mut self.world, context, &self.level_manager)
                .unwrap();
            self.input_handler.update().unwrap();
            self.system_manager.update(&self.world, context).unwrap();
            self.audio_manager.run().unwrap();
            self.navigation.update().unwrap();
            self.world.update().unwrap();
        }

        if timer::ticks(context) % 500 == 0 {
            dbg!(timer::fps(context));
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);
        self.system_manager
            .display(
                &self.world,
                context,
                &self.image_manager,
                &self.level_manager.get_level(),
            )
            .unwrap();

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

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, button: Button, _id: GamepadId) {
        self.input_handler
            .handle_stop_controller_input(button, &self.world, &mut self.navigation)
            .unwrap();
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.input_handler
            .handle_keyboard_input(keycode, &mut self.navigation)
            .unwrap();
    }
}
