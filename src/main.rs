use eyre::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};
use platformer_with_level_editor::MainState;

const GAME_NAME: &str = "Name_of_Game";

fn main() -> Result<()> {
    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(false);
    let window_mode = WindowMode::default()
        .dimensions(1920.0, 1080.0)
        .fullscreen_type(ggez::conf::FullscreenType::Desktop)
        .maximized(true);
    let (mut context, mut event_loop) = ContextBuilder::new("bbplatformer", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;
    let mut main_state = MainState::new(GAME_NAME);

    main_state.setup(&mut context)?;
    event::run(&mut context, &mut event_loop, &mut main_state)?;
    Ok(())
}
