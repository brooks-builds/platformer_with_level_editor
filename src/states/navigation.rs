use strum_macros::ToString;

#[derive(Clone, Copy, Debug, ToString)]
pub enum Navigation {
    Credits,
    SelectLevel,
    Settings,
    TitleScreen,
}

impl Default for Navigation {
    fn default() -> Self {
        Self::TitleScreen
    }
}

impl From<String> for Navigation {
    fn from(string: String) -> Self {
        match string.to_lowercase().as_ref() {
            "credits" => Self::Credits,
            "selectlevel" => Self::SelectLevel,
            "settings" => Self::Settings,
            "titlescreen" => Self::TitleScreen,
            _ => Self::default(),
        }
    }
}
