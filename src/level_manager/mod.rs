use self::level::Level;

pub mod level;

pub struct LevelManager {
    level: Level,
}

impl LevelManager {
    pub fn new() -> Self {
        let level = Level::new();
        Self { level }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }
}
