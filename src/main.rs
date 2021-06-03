use eyre::Result;
use ggez::conf::{Backend, WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};
use platformer_with_level_editor::MainState;

const GAME_NAME: &str = "Name_of_Game";

fn main() -> Result<()> {
    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(true);
    let window_mode = WindowMode::default()
        .dimensions(1920.0, 1080.0)
        .fullscreen_type(ggez::conf::FullscreenType::Windowed)
        .max_dimensions(1920.0, 1080.0)
        .maximized(true);
    let (mut context, mut event_loop) = ContextBuilder::new("bbplatformer", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .backend(Backend::default().version(4, 6).gl())
        .build()?;
    let mut main_state = MainState::new(&mut context)?;

    main_state.setup(&mut context, GAME_NAME)?;
    event::run(&mut context, &mut event_loop, &mut main_state)?;
    Ok(())
}
