use bbecs::world::{World, WorldMethods};
use crossbeam::channel::Sender;
use events::event::Event;
use events::EventManager;
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, Context};
use loaders::LoaderManager;
use names::component_names::ComponentNames;
use names::resource_names::ResourceNames;
use states::navigation::Navigation;
use system_manager::SystemManager;

mod events;
mod helpers;
mod loaders;
mod names;
mod states;
mod system_manager;

pub struct MainState {
    world: World,
    event_manager: EventManager,
    event_sender: Sender<Event>,
    loader_manager: LoaderManager,
    system_manager: SystemManager,
}

impl MainState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&mut self, _context: &mut Context, game_name: &str) -> Result<()> {
        self.world
            .add_resource(ResourceNames::GameName.to_string(), game_name.to_owned());
        self.world
            .add_resource(ResourceNames::TitleFontSize.to_string(), 72.0_f32);
        self.world
            .add_resource(ResourceNames::FontSize.to_string(), 24.0_f32);
        self.event_sender
            .send(Event::NavigatingTo(Navigation::TitleScreen))?;

        self.world.register(ComponentNames::Text.to_string())?;
        self.world.register(ComponentNames::Position.to_string())?;
        self.world.register(ComponentNames::Selected.to_string())?;
        self.world
            .register(ComponentNames::Selectable.to_string())?;
        self.world
            .register(ComponentNames::TextFragment.to_string())?;
        Ok(())
    }
}

impl Default for MainState {
    fn default() -> Self {
        let world = World::new();
        let mut event_manager = EventManager::new();
        let event_sender = event_manager.register();
        let loader_manager = LoaderManager::new(&mut event_manager);
        let system_manager = SystemManager::new();

        Self {
            world,
            event_manager,
            event_sender,
            loader_manager,
            system_manager,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        self.event_manager.process().unwrap();
        self.loader_manager
            .update(&mut self.world, context)
            .unwrap();
        self.system_manager.update(&self.world, context).unwrap();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);
        self.system_manager.display(&self.world, context).unwrap();

        graphics::present(context)
    }
}
