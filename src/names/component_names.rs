use strum_macros::{AsRefStr, ToString};

#[derive(AsRefStr, ToString)]
pub enum ComponentNames {
    NavigateTo,
    Position,
    Selectable,
    Selected,
    Text,
    TextFragment,
}
