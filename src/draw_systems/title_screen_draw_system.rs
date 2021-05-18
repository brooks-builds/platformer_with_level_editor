use eyre::Result;
use ggez::graphics::{self, Scale, Text, TextFragment, WHITE};
use ggez::Context;

pub struct TitleScreenDrawSystem {
    title: TextFragment,
    play: TextFragment,
    settings: TextFragment,
}

impl TitleScreenDrawSystem {
    pub fn new(game_title: &str) -> Self {
        let title = TextFragment::new(game_title)
            .color(WHITE)
            .scale(Scale::uniform(72.0));
        let play = TextFragment::new("Play")
            .color(WHITE)
            .scale(Scale::uniform(24.0));
        let settings = TextFragment::new("Settings")
            .color(WHITE)
            .scale(Scale::uniform(24.0));

        Self {
            title,
            play,
            settings,
        }
    }

    pub fn run(&self, context: &mut Context) {
        let title = Text::new(self.title.clone());
        let screen_coordinates = graphics::screen_coordinates(context);
        let title_destination = [
            screen_coordinates.w / 2.0 - title.width(context) as f32 / 2.0,
            title.height(context) as f32 * 2.0,
        ];
        graphics::queue_text(context, &title, title_destination, None);

        let play = Text::new(self.play.clone());
        let play_destination = [
            screen_coordinates.w / 2.0 - play.width(context) as f32 / 2.0,
            screen_coordinates.h - play.height(context) as f32 * 10.0,
        ];
        graphics::queue_text(context, &play, play_destination, None);

        let settings = Text::new(self.settings.clone());
        let settings_destination = [
            screen_coordinates.w / 2.0 - settings.width(context) as f32 / 2.0,
            screen_coordinates.h - settings.height(context) as f32 * 8.0,
        ];
        graphics::queue_text(context, &settings, settings_destination, None);
    }
}
