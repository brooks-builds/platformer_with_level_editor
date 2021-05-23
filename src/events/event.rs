use crate::command::Command;
use crate::states::navigation::Navigation;
use strum_macros::{AsRefStr, ToString};

#[derive(ToString, AsRefStr, Clone, Copy)]
pub enum Event {
    NavigatingTo(Navigation),
    Command(Command),
    ChangeMenuItem,
}
