use super::Loader;

pub struct PlayLoader;

impl Loader for PlayLoader {
    fn load(
        &mut self,
        world: &mut bbecs::world::World,
        context: &mut ggez::Context,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
