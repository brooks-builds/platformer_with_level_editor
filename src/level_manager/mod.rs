use self::level::Level;

pub mod level;

pub struct LevelManager {
    level: Level,
}

impl LevelManager {
    pub fn new() -> Self {
        let mut level = Level::new();
        level.add_floor();
        Self { level }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }
}
