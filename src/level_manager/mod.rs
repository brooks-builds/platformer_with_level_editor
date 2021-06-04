use std::collections::HashMap;

use crossbeam::channel::Receiver;
use eyre::Result;

use crate::{
    events::{event::Event, EventManager},
    level_manager::level::GridCoordinate,
};

use self::level::Level;

pub mod level;

pub struct LevelManager {
    levels: HashMap<String, Level>,
    event_receiver: Receiver<Event>,
}

impl LevelManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let mut level = Level::new("Introduction".into(), 50, 10);
        level.add_floor();
        level.add_end(4, 8);
        let level2 = Level::new("No Floor".into(), 10, 30);
        let mut levels = HashMap::new();
        levels.insert(level.name.clone(), level);
        levels.insert(level2.name.clone(), level2);
        let event_receiver = event_manager.subscribe(vec![
            Event::InsertIntoLevel(
                GridCoordinate::default(),
                level::Entity::Platform,
                "".to_string(),
            )
            .to_string(),
            Event::RemoveFromLevel(GridCoordinate::default(), String::new()).to_string(),
        ]);

        Self {
            levels,
            event_receiver,
        }
    }

    pub fn get_level(&self, name: &str) -> Option<&Level> {
        self.levels.get(name)
    }

    pub fn get_all_levels(&self) -> Vec<&Level> {
        self.levels.values().collect()
    }

    pub fn update(&mut self) -> Result<()> {
        match self.event_receiver.try_recv() {
            Ok(Event::InsertIntoLevel(grid_coordinate, entity, level_name)) => {
                let level = self.levels.get_mut(&level_name).unwrap();
                dbg!("inserting");
                level.map.insert(grid_coordinate, entity);
            }
            Ok(Event::RemoveFromLevel(grid_coordinate, level_name)) => {
                dbg!("removing");
                let level = self.levels.get_mut(&level_name).unwrap();
                level.map.remove(&grid_coordinate);
            }
            _ => {}
        }
        Ok(())
    }
}
