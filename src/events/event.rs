use crate::{command::Command, navigation::screens::NavigationScreens};
use strum_macros::{AsRefStr, ToString};

#[derive(ToString, AsRefStr, Clone)]
pub enum Event {
    NavigatingTo(NavigationScreens),
    Command(Command),
    ChangeMenuItem,
}
