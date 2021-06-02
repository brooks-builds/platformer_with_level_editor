use strum_macros::ToString;

#[derive(Debug, Clone, ToString)]
pub enum NavigationScreens {
    Title,
    LevelSelect,
    Settings,
    Credits,
    Unknown,
    Play(String),
    EditLevel,
}
