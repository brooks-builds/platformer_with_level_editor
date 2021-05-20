#[derive(Clone, Copy)]
pub enum Navigation {
    TitleScreen,
    _Settings,
}

impl Default for Navigation {
    fn default() -> Self {
        Self::TitleScreen
    }
}
