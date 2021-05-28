use std::cell::Ref;

use bbecs::resources::resource::ResourceCast;
use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    get_resource, query,
    world::{DataWrapper, World},
};
use eyre::Result;
use ggez::graphics::pipe::Data;
use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, MeshBuilder},
    Context,
};

use crate::names::{component_names::ComponentNames, resource_names::ResourceNames};

pub struct CameraSystem;

impl CameraSystem {
    pub fn run(&self, world: &World, context: &mut Context) -> Result<()> {
        let mut query;
        let (cameras, camera_positions, camera_widths, camera_heights) = query!(
            world,
            query,
            ComponentNames::Camera.as_ref(),
            ComponentNames::Position.as_ref(),
            ComponentNames::Width.as_ref(),
            ComponentNames::Height.as_ref()
        );
        if camera_positions.is_empty() {
            return Ok(());
        }
        let wrapped_camera_position: &DataWrapper<Point> = camera_positions[0].cast()?;
        let wrapped_camera_width: &DataWrapper<f32> = camera_widths[0].cast()?;
        let wrapped_camera_height: &DataWrapper<f32> = camera_heights[0].cast()?;
        let (positions,) = query!(world, query, ComponentNames::Position.as_ref());
        let mut mesh_builder = MeshBuilder::new();

        graphics::push_transform(
            context,
            Some(
                DrawParam::new()
                    .dest([
                        -(wrapped_camera_position.borrow().x
                            - *wrapped_camera_width.borrow() / 2.0),
                        wrapped_camera_position.borrow().y - *wrapped_camera_height.borrow() / 2.0,
                    ])
                    .to_matrix(),
            ),
        );
        for (_index, position) in positions.iter().enumerate() {
            let wrapped_position: &DataWrapper<Point> = position.cast()?;

            mesh_builder.circle(
                DrawMode::fill(),
                wrapped_position.borrow().to_array(),
                10.0,
                0.1,
                Color::new(0.0, 1.0, 0.0, 1.0),
            );
        }

        graphics::apply_transformations(context)?;

        if let Ok(mesh) = mesh_builder.build(context) {
            graphics::draw(context, &mesh, DrawParam::new())?;
        }
        graphics::pop_transform(context);

        Ok(())
    }
}
