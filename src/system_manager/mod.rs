use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use self::draw_text::DrawTextSystem;

mod draw_text;

#[derive(Default)]
pub struct SystemManager {
    draw_text: DrawTextSystem,
}

impl SystemManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self, world: &World, context: &mut Context) -> Result<()> {
        self.draw_text.run(world, context)?;
        Ok(())
    }
}
