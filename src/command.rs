#[derive(Debug, Clone, Copy)]
pub enum Command {
    SelectUp,
    SelectDown,
    StartMovingRight,
    StartMovingLeft,
    StopMovingRight,
    StopMovingLeft,
}
