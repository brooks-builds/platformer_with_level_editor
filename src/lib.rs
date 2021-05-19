use bbecs::world::{World, WorldMethods};
use crossbeam::channel::Sender;
use events::event::Event;
use events::EventManager;
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Font, Scale, Text, TextFragment, BLACK, WHITE};
use ggez::{graphics, Context};
use loaders::LoaderManager;
use states::navigation::Navigation;

mod events;
mod loaders;
mod states;

pub struct MainState {
    navigation: Navigation,
    world: World,
    event_manager: EventManager,
    event_sender: Sender<Event>,
    loader_manager: LoaderManager,
}

impl MainState {
    pub fn new() -> Self {
        let navigation = Navigation::default();
        let world = World::new();
        let mut event_manager = EventManager::new();
        let event_sender = event_manager.register();
        let loader_manager = LoaderManager::new(&mut event_manager);

        Self {
            navigation,
            world,
            event_manager,
            event_sender,
            loader_manager,
        }
    }

    pub fn setup(&mut self, _context: &mut Context, game_name: &str) -> Result<()> {
        self.world
            .add_resource("game name".into(), game_name.to_owned());
        self.world.add_resource("title font size".into(), 72.0_f32);
        self.event_sender
            .send(Event::NavigatingTo(Navigation::TitleScreen))?;
        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        self.event_manager.process().unwrap();
        self.loader_manager.update(&mut self.world).unwrap();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);

        graphics::present(context)
    }
}
