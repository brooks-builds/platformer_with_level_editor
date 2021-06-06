use bbecs::world::World;
use eyre::Result;
use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, MeshBuilder, Rect},
    Context,
};

use crate::helpers::{
    game_entity::GameEntity, query_platforms::query_platforms, query_player::query_player,
};

pub struct DisplayCollisionBorderSystem;

impl DisplayCollisionBorderSystem {
    pub fn run(&self, world: &World, context: &mut Context) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };
        let platforms = query_platforms(world)?;

        let mut mesh_builder = MeshBuilder::new();
        let color = Color::new(1.0, 0.0, 0.0, 1.0);

        self.draw_border_around_game_entity(&player, &mut mesh_builder, color);
        platforms.iter().for_each(|game_entity| {
            self.draw_border_around_game_entity(game_entity, &mut mesh_builder, color)
        });

        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;
        Ok(())
    }

    fn draw_border_around_game_entity(
        &self,
        game_entity: &GameEntity,
        mesh_builder: &mut MeshBuilder,
        color: Color,
    ) {
        mesh_builder.circle(
            DrawMode::fill(),
            game_entity.position.borrow().to_array(),
            5.0,
            0.1,
            color,
        );
        let rect = Rect::new(
            game_entity.left(),
            game_entity.top(),
            game_entity.width,
            game_entity.height,
        );
        mesh_builder.rectangle(DrawMode::stroke(1.0), rect, color);
    }
}
