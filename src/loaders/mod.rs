use std::u32;

use bbecs::components::CastComponents;
use bbecs::query;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::Context;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::navigation::screens::NavigationScreens;

use self::play::PlayLoader;
use self::select_level::SelectLevelLoader;
use self::settings::SettingsLoader;
use self::title_screen::TitleScreenLoader;

mod insert_into_world;
mod play;
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
    play: PlayLoader,
}

impl LoaderManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(NavigationScreens::Title).to_string()
            ]);
        let title_screen = TitleScreenLoader;
        let select_level = SelectLevelLoader;
        let settings = SettingsLoader;
        let play = PlayLoader;

        Self {
            event_receiver,
            title_screen,
            select_level,
            settings,
            play,
        }
    }

    pub fn setup(&mut self, world: &mut World, context: &mut Context) -> Result<()> {
        self.title_screen.load(world, context)?;
        Ok(())
    }

    pub fn update(&mut self, world: &mut World, context: &mut Context) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Event::NavigatingTo(target) = event {
                self.clear_world(world)?;

                match target {
                    NavigationScreens::Title => self.title_screen.load(world, context)?,
                    NavigationScreens::LevelSelect => self.select_level.load(world, context)?,
                    NavigationScreens::Settings => self.settings.load(world, context)?,
                    NavigationScreens::Credits => {}
                    NavigationScreens::Unknown => {}
                    NavigationScreens::Play => self.play.load(world, context)?,
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
