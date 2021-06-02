use strum_macros::{AsRefStr, ToString};
#[derive(AsRefStr, ToString)]
pub enum ResourceNames {
    GameName,
    TitleFontSize,
    FontSize,
    UnitSize,
    Gravity,
    PlayerMoveSpeed,
    JumpForce,
}
