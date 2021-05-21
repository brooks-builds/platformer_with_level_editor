use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::{self, DrawParam, Text, WHITE};
use ggez::Context;

use crate::names::component_names::ComponentNames;

#[derive(Default)]
pub struct DrawText;

impl DrawText {
    pub fn run(&self, world: &World, context: &mut Context) -> Result<()> {
        let query;
        let (text_fragments, positions) = query!(
            world,
            query,
            ComponentNames::Text.as_ref(),
            ComponentNames::Position.as_ref()
        );

        for (index, text_fragment) in text_fragments.iter().enumerate() {
            let text: &DataWrapper<Text> = text_fragment.cast()?;
            let position: &DataWrapper<Point> = positions[index].cast()?;

            graphics::draw(
                context,
                &*text.borrow(),
                DrawParam::new()
                    .dest(position.borrow().to_array())
                    .color(WHITE),
            )?;
        }

        Ok(())
    }
}
