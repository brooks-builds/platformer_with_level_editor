use strum_macros::{AsRefStr, ToString};

#[derive(AsRefStr, ToString)]
pub enum ComponentNames {
    Text,
    TextFragment,
    Position,
    Selected,
    Selectable,
}
