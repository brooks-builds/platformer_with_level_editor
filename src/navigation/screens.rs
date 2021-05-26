use strum_macros::ToString;

#[derive(Debug, Clone, Copy, ToString)]
pub enum NavigationScreens {
    Title,
    LevelSelect,
    Settings,
    Credits,
    Unknown,
}
