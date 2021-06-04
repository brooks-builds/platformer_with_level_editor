use bbecs::{data_types::point::Point, world::World};
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::{
    graphics::{self, DrawParam, MeshBuilder, WHITE},
    Context,
};

use crate::{
    events::{event::Event, EventManager},
    level_manager::{level::Level, LevelManager},
    navigation::screens::NavigationScreens,
};

pub struct DrawEditingLevel {
    event_receiver: Receiver<Event>,
    level_name: Option<String>,
}

impl DrawEditingLevel {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager.subscribe(vec![Event::NavigatingTo(
            NavigationScreens::EditLevel("".to_string()),
        )
        .to_string()]);
        let level_name = None;

        Self {
            event_receiver,
            level_name,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        match self.event_receiver.try_recv() {
            Ok(Event::NavigatingTo(NavigationScreens::EditLevel(level_name))) => {
                self.level_name = Some(level_name)
            }
            Ok(Event::NavigatingTo(_)) => self.level_name = None,
            _ => {}
        }

        Ok(())
    }

    pub fn run(
        &mut self,
        context: &mut Context,
        level_manager: &LevelManager,
        _world: &World,
    ) -> Result<()> {
        // We want to check if the level name exists. The only reason it doesn't is if we aren't in editing mode.
        // So we want to exit early if we are in anything other than editing mode.

        // This pattern allows us to get the level name out and also handle the none case at the same time.
        let level_name = if let Some(level_name) = &self.level_name {
            level_name
        } else {
            return Ok(());
        };

        let level = level_manager.get_level(level_name).unwrap();
        let cell_size = self.calculate_cell_size(level, context);

        let mut mesh_builder = MeshBuilder::new();
        let line_height = level.height as f32 * cell_size;
        let line_width = level.width as f32 * cell_size;

        for count in 0..=level.width {
            let start = ggez::mint::Point2 {
                x: count as f32 * cell_size,
                y: 0.0,
            };
            let end = ggez::mint::Point2 {
                x: start.x,
                y: start.y + line_height,
            };
            mesh_builder.line(&[start, end], 1.0, WHITE)?;
        }

        for count in 0..=level.height {
            let start = Point::new(0.0, count as f32 * cell_size);
            let end = Point::new(start.x + line_width, start.y);

            mesh_builder.line(&[start.to_array(), end.to_array()], 1.0, WHITE)?;
        }

        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;

        Ok(())
    }

    fn calculate_cell_size(&self, level: &Level, context: &mut Context) -> f32 {
        let screen_coordinates = graphics::screen_coordinates(context);
        let width = screen_coordinates.w / level.width as f32;
        let height = screen_coordinates.h / level.height as f32;

        if width < height {
            width
        } else {
            height
        }
    }
}
