use std::u32;

use bbecs::components::CastComponents;
use bbecs::query;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::Context;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::states::navigation::Navigation;

use self::select_level::SelectLevelLoader;
use self::settings::SettingsLoader;
use self::title_screen::TitleScreenLoader;

mod insert_into_world;
mod select_level;
mod settings;
mod title_screen;

trait Loader {
    fn load(&mut self, world: &mut World, context: &mut Context) -> Result<()>;
}

pub struct LoaderManager {
    event_receiver: Receiver<Event>,
    title_screen: TitleScreenLoader,
    select_level: SelectLevelLoader,
    settings: SettingsLoader,
}

impl LoaderManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(Navigation::TitleScreen).to_string()
            ]);
        let title_screen = TitleScreenLoader;
        let select_level = SelectLevelLoader;
        let settings = SettingsLoader;

        Self {
            event_receiver,
            title_screen,
            select_level,
            settings,
        }
    }

    pub fn update(&mut self, world: &mut World, context: &mut Context) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Event::NavigatingTo(target) = event {
                self.clear_world(world)?;

                match target {
                    Navigation::TitleScreen => self.title_screen.load(world, context)?,
                    Navigation::SelectLevel => self.select_level.load(world, context)?,
                    Navigation::Credits => {}
                    Navigation::Settings => self.settings.load(world, context)?,
                }
            }
        }
        Ok(())
    }

    fn clear_world(&self, world: &mut World) -> Result<()> {
        let query;
        let (ids,) = query!(world, query, ENTITY_ID);
        for id in ids {
            let wrapped_id: &DataWrapper<u32> = id.cast()?;
            world.delete_by_id(*wrapped_id.borrow())?;
        }
        Ok(())
    }
}
