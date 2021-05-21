use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use self::draw_selectable_text::DrawText;
use self::update_text::UpdateTextSystem;

mod draw_selectable_text;
mod update_text;

#[derive(Default)]
pub struct SystemManager {
    draw_text: DrawText,
    update_text: UpdateTextSystem,
}

impl SystemManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&self, world: &World, context: &mut Context) -> Result<()> {
        self.update_text.run(world, context)?;
        Ok(())
    }

    pub fn display(&self, world: &World, context: &mut Context) -> Result<()> {
        self.draw_text.run(world, context)?;
        Ok(())
    }
}
