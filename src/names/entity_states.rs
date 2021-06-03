use strum_macros::{AsRefStr, ToString};

#[derive(AsRefStr, ToString, PartialEq, Clone, Copy)]
pub enum EntityStates {
    Falling,
    Standing,
    Unknown,
}

impl EntityStates {
    pub fn from_str(string: &str) -> Self {
        match string {
            "Falling" => Self::Falling,
            "Standing" => Self::Standing,
            _ => Self::Unknown,
        }
    }
}
