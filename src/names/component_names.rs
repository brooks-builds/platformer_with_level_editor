use strum_macros::{AsRefStr, ToString};

#[derive(AsRefStr, ToString)]
pub enum ComponentNames {
    Text,
    Position,
}
