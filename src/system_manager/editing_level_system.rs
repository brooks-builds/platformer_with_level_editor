use bbecs::data_types::point::Point;
use crossbeam::channel::{Receiver, Sender};
use eyre::Result;
use ggez::{graphics, Context};

use crate::{
    events::{event::Event, EventManager},
    level_manager::{
        level::{Entity, GridCoordinate, Level},
        LevelManager,
    },
    navigation::screens::NavigationScreens,
};

pub struct EditingLevelSystem {
    event_receiver: Receiver<Event>,
    level_name: Option<String>,
    mouse_position: Option<Point>,
    event_sender: Sender<Event>,
}

impl EditingLevelSystem {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager.subscribe(vec![
            Event::MouseClicked(Point::new(0.0, 0.0)).to_string(),
            Event::NavigatingTo(NavigationScreens::Unknown).to_string(),
        ]);
        let level_name = None;
        let mouse_position = None;
        let event_sender = event_manager.register();

        Self {
            event_receiver,
            level_name,
            mouse_position,
            event_sender,
        }
    }

    pub fn run(&mut self, level_manager: &LevelManager, context: &mut Context) -> Result<()> {
        self.handle_events()?;

        let level_name = if let Some(level_name) = &self.level_name {
            level_name.clone()
        } else {
            return Ok(());
        };

        let clicked_position = if let Some(position) = self.mouse_position {
            position
        } else {
            return Ok(());
        };

        let level = level_manager.get_level(&level_name).unwrap();
        let grid_coordinate =
            self.convert_position_to_grid_coordinate(clicked_position, level, context);
        let entity = Entity::Platform;

        self.event_sender
            .send(Event::InsertIntoLevel(grid_coordinate, entity, level_name))?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match self.event_receiver.try_recv() {
            Ok(Event::NavigatingTo(NavigationScreens::EditLevel(level_name))) => {
                self.level_name = Some(level_name)
            }
            Ok(Event::MouseClicked(mouse_position)) => self.mouse_position = Some(mouse_position),
            _ => {}
        }
        Ok(())
    }

    fn convert_position_to_grid_coordinate(
        &self,
        position: Point,
        level: &Level,
        context: &mut Context,
    ) -> GridCoordinate {
        let cell_size = self.calculate_cell_size(level, context);
        let x = (position.x / cell_size as f32) as u32;
        let y = (position.y / cell_size as f32) as u32;

        GridCoordinate { x, y }
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
