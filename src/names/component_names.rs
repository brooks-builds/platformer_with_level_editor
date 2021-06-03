use strum_macros::{AsRefStr, ToString};

#[derive(AsRefStr, ToString)]
pub enum ComponentNames {
    NavigateTo,
    Position,
    Selectable,
    Selected,
    Text,
    TextFragment,
    Velocity,
    Acceleration,
    AffectedByGravity,
    Camera,
    Width,
    Height,
    Platform,
    ImageName,
    Player,
    Name,
    EntityState,
}
