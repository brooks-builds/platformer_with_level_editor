use self::level::Level;

pub mod level;

pub struct LevelManager {
    levels: Vec<Level>,
}

impl LevelManager {
    pub fn new() -> Self {
        let mut level = Level::new("Introduction".into());
        level.add_floor();
        let mut level2 = Level::new("Meow".into());
        level2.add_floor();
        let levels = vec![level, level2];
        Self { levels }
    }

    pub fn get_level(&self) -> &Level {
        &self.levels[0]
    }

    pub fn get_all_levels(&self) -> &[Level] {
        &self.levels
    }
}
