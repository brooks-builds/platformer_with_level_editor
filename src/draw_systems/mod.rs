use eyre::Result;
use ggez::graphics::{DrawParam, FilterMode};
use ggez::{graphics, Context};

use crate::states::navigation::Navigation;

use self::title_screen_draw_system::TitleScreenDrawSystem;

mod title_screen_draw_system;

pub struct DrawSystems {
    title_screen: TitleScreenDrawSystem,
}

impl DrawSystems {
    pub fn new(game_title: &str) -> Self {
        let title_screen = TitleScreenDrawSystem::new(game_title);
        Self { title_screen }
    }

    pub fn run(&self, context: &mut Context, navigation: &Navigation) -> Result<()> {
        match navigation {
            Navigation::TitleScreen => {
                self.title_screen.run(context);
            }
            Navigation::Settings => {}
        }

        graphics::draw_queued_text(context, DrawParam::new(), None, FilterMode::Linear)?;
        Ok(())
    }
}
