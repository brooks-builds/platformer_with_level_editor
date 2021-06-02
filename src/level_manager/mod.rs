use std::collections::HashMap;

use self::level::Level;

pub mod level;

pub struct LevelManager {
    levels: HashMap<String, Level>,
}

impl LevelManager {
    pub fn new() -> Self {
        let mut level = Level::new("Introduction".into());
        level.add_floor();
        let level2 = Level::new("No Floor".into());
        let mut levels = HashMap::new();
        levels.insert(level.name.clone(), level);
        levels.insert(level2.name.clone(), level2);

        Self { levels }
    }

    pub fn get_level(&self, name: &str) -> Option<&Level> {
        self.levels.get(name)
    }

    pub fn get_all_levels(&self) -> Vec<&Level> {
        self.levels.values().collect()
    }
}
