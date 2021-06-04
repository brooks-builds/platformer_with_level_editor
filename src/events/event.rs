use crate::{
    command::Command,
    level_manager::level::{Entity, GridCoordinate},
    navigation::screens::NavigationScreens,
};
use bbecs::data_types::point::Point;
use strum_macros::{AsRefStr, ToString};

#[derive(ToString, AsRefStr, Clone)]
pub enum Event {
    NavigatingTo(NavigationScreens),
    Command(Command),
    ChangeMenuItem,
    Won,
    MouseClicked(Point),
    InsertIntoLevel(GridCoordinate, Entity, String),
}
