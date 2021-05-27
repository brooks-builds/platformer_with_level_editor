use std::cell::Ref;

use bbecs::resources::resource::ResourceCast;
use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    get_resource, query,
    world::{DataWrapper, World},
};
use eyre::Result;
use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, MeshBuilder},
    Context,
};

use crate::names::{component_names::ComponentNames, resource_names::ResourceNames};

pub struct DrawEntities;

impl DrawEntities {
    pub fn run(&self, world: &World, context: &mut Context) -> Result<()> {
        let query;
        let (positions,) = query!(world, query, ComponentNames::Position.as_ref());
        let mut mesh_builder = MeshBuilder::new();
        for (index, position) in positions.iter().enumerate() {
            let wrapped_position: &DataWrapper<Point> = position.cast()?;
            let screen_position =
                self.convert_world_position_to_pixels(wrapped_position.borrow(), world);
            mesh_builder.circle(
                DrawMode::fill(),
                screen_position.to_array(),
                10.0,
                0.1,
                Color::new(0.0, 1.0, 0.0, 1.0),
            );
        }
        if let Ok(mesh) = mesh_builder.build(context) {
            graphics::draw(context, &mesh, DrawParam::new())?;
        }

        Ok(())
    }

    fn convert_world_position_to_pixels(&self, world_position: Ref<Point>, world: &World) -> Point {
        let unit_size: &Point;
        get_resource!(unit_size, world, ResourceNames::UnitSize.as_ref());
        Point::new(
            world_position.x * unit_size.x,
            world_position.y * unit_size.y,
        )
    }
}
