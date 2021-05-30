use std::cell::Ref;

use bbecs::{
    components::CastComponents,
    data_types::point::Point,
    query,
    world::{DataWrapper, World},
};
use eyre::Result;
use ggez::{
    graphics::{self, DrawParam},
    Context,
};

use crate::image_manager::ImageManager;
use crate::names::component_names::ComponentNames;

pub struct CameraSystem;

impl CameraSystem {
    pub fn run(
        &self,
        world: &World,
        context: &mut Context,
        image_manager: &ImageManager,
    ) -> Result<()> {
        let query;
        let (_cameras, camera_positions, camera_widths, camera_heights) = query!(
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
        self.draw_entities(world, context, image_manager)?;
        graphics::pop_transform(context);

        Ok(())
    }

    pub fn draw_entities(
        &self,
        world: &World,
        context: &mut Context,
        image_manager: &ImageManager,
    ) -> Result<()> {
        let query;
        let (positions, widths, heights, image_names) = query!(
            world,
            query,
            ComponentNames::Position.as_ref(),
            ComponentNames::Width.as_ref(),
            ComponentNames::Height.as_ref(),
            ComponentNames::ImageName.as_ref()
        );
        for (index, position) in positions.iter().enumerate() {
            let wrapped_position: &DataWrapper<Point> = position.cast()?;
            let width: &DataWrapper<f32> = widths[index].cast()?;
            let height: &DataWrapper<f32> = heights[index].cast()?;
            let image_name: &DataWrapper<String> = image_names[index].cast()?;

            let destination = self.calculate_destination(
                wrapped_position.borrow(),
                *width.borrow(),
                *height.borrow(),
            );

            if let Some(image) = image_manager.get_image_from_str(&*image_name.borrow()) {
                graphics::draw(
                    context,
                    image,
                    DrawParam::new().dest(destination.to_array()),
                )?;
            }
        }
        Ok(())
    }

    fn calculate_destination(&self, position: Ref<Point>, width: f32, height: f32) -> Point {
        Point::new(position.x - width / 2.0, position.y - height / 2.0)
    }
}
