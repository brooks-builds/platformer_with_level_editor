use crate::states::navigation::Navigation;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    SelectUp,
    SelectDown,
}
