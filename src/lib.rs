use draw_systems::DrawSystems;
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Font, Scale, Text, TextFragment, BLACK, WHITE};
use ggez::{graphics, Context};
use states::navigation::Navigation;

mod draw_systems;
mod states;

pub struct MainState {
    navigation: Navigation,
    draw_systems: DrawSystems,
}

impl MainState {
    pub fn new(game_name: &str) -> Self {
        let navigation = Navigation::default();
        let draw_systems = DrawSystems::new(game_name);
        Self {
            navigation,
            draw_systems,
        }
    }

    pub fn setup(&mut self, _context: &mut Context) -> Result<()> {
        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);

        self.draw_systems.run(context, &self.navigation).unwrap();

        graphics::present(context)
    }
}
