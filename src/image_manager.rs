use eyre::Result;
use ggez::{graphics::Image, Context};
use std::collections::HashMap;
use strum_macros::{AsRefStr, ToString};

#[derive(Debug, Eq, PartialEq, Hash, ToString, AsRefStr)]
pub enum ImageName {
    GrassMiddle,
    Player,
}

impl ImageName {
    pub fn new_from_str(string: &str) -> Option<Self> {
        match string {
            "GrassMiddle" => Some(Self::GrassMiddle),
            "Player" => Some(Self::Player),
            _ => None,
        }
    }
}

pub struct ImageManager {
    images: HashMap<ImageName, Image>,
}

impl ImageManager {
    pub fn new(context: &mut Context) -> Result<Self> {
        let grass_middle_image = Image::new(context, "/grassMid.png")?;
        let player_image = Image::new(context, "/bunny1_ready.png")?;
        let mut images = HashMap::new();
        images.insert(ImageName::GrassMiddle, grass_middle_image);
        images.insert(ImageName::Player, player_image);

        Ok(Self { images })
    }

    pub fn get_image_from_str(&self, name: &str) -> Option<&Image> {
        let image_name = ImageName::new_from_str(name)?;
        self.images.get(&image_name)
    }
}
