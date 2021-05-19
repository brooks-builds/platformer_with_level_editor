use bbecs::get_resource;
use bbecs::world::{World, WorldMethods};
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::graphics::{Scale, Text, TextFragment, WHITE};

use crate::events::event::Event;
use crate::events::EventManager;
use crate::states::navigation::Navigation;
use bbecs::resources::resource::ResourceCast;

pub struct LoaderManager {
    event_receiver: Receiver<Event>,
}

impl LoaderManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(Navigation::TitleScreen).to_string()
            ]);

        Self { event_receiver }
    }

    pub fn update(&mut self, world: &mut World) -> Result<()> {
        while let Ok(event) = self.event_receiver.try_recv() {
            if let Event::NavigatingTo(target) = event {
                match target {
                    Navigation::TitleScreen => self.load_title_screen(world)?,
                    Navigation::Settings => {}
                }
            }
        }
        Ok(())
    }

    fn load_title_screen(&mut self, world: &mut World) -> Result<()> {
        let title = {
            let game_name: &String;
            get_resource!(game_name, world, "game name");
            let title_font_size: &f32;
            get_resource!(title_font_size, world, "title font size");
            Text::new(
                TextFragment::new(game_name.clone())
                    .color(WHITE)
                    .scale(Scale::uniform(*title_font_size)),
            )
        };

        world.spawn_entity()?.with_component("text", title)?;
        Ok(())
    }
}
