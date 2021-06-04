use std::u32;

use bbecs::components::CastComponents;
use bbecs::query;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::Context;

use crate::events::event::Event;
use crate::events::EventManager;
use crate::level_manager::LevelManager;
use crate::navigation::screens::NavigationScreens;

use self::edit_scene::EditScene;
use self::play::PlayLoader;
use self::select_level_loader::SelectLevelLoader;
use self::settings::SettingsLoader;
use self::title_screen_loader::TitleScreenLoader;
use self::won_loader::WonLoader;

mod edit_scene;
mod insert_into_world;
mod play;
mod select_level_loader;
mod settings;
mod title_screen_loader;
mod won_loader;

pub struct LoaderManager {
    event_receiver: Receiver<Event>,
    title_screen: TitleScreenLoader,
    select_level: SelectLevelLoader,
    settings: SettingsLoader,
    play: PlayLoader,
    edit_scene: EditScene,
    won_loader: WonLoader,
}

impl LoaderManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager.subscribe(vec![
            Event::NavigatingTo(NavigationScreens::Title).to_string(),
            Event::Won.to_string(),
        ]);
        let title_screen = TitleScreenLoader;
        let select_level = SelectLevelLoader;
        let settings = SettingsLoader;
        let play = PlayLoader;
        let edit_scene = EditScene;
        let won_loader = WonLoader;

        Self {
            event_receiver,
            title_screen,
            select_level,
            settings,
            play,
            edit_scene,
            won_loader,
        }
    }

    pub fn setup(
        &mut self,
        world: &mut World,
        context: &mut Context,
        level_manager: &LevelManager,
    ) -> Result<()> {
        self.title_screen.load(world, context, level_manager)?;
        Ok(())
    }

    pub fn update(
        &mut self,
        world: &mut World,
        context: &mut Context,
        level_manager: &LevelManager,
    ) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Event::NavigatingTo(target) = event.clone() {
                self.clear_world(world)?;
                dbg!(&target);

                match target {
                    NavigationScreens::Title => {
                        self.title_screen.load(world, context, level_manager)?
                    }
                    NavigationScreens::LevelSelect => {
                        self.select_level.load(world, context, level_manager)?
                    }
                    NavigationScreens::Settings => {
                        self.settings.load(world, context, level_manager)?
                    }
                    NavigationScreens::Credits => {}
                    NavigationScreens::Unknown => {}
                    NavigationScreens::Play(level_name) => {
                        self.play.load(world, level_manager, level_name)?
                    }
                    NavigationScreens::EditLevel(level_name) => {
                        self.edit_scene
                            .load(world, context, level_manager, &level_name)?
                    }
                }
            }

            if let Event::Won = event {
                self.won_loader.run(world)?;
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
