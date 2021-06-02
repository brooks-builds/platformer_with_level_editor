use bbecs::{
    components::CastComponents,
    query,
    world::{DataWrapper, World},
};
use crossbeam::channel::Receiver;
use eyre::Result;
use ggez::{
    graphics::{self, DrawParam, MeshBuilder, WHITE},
    Context,
};

use crate::{
    events::{event::Event, EventManager},
    level_manager::{level::Level, LevelManager},
    names::component_names::ComponentNames,
    navigation::screens::NavigationScreens,
};

pub struct DrawEditingLevel {
    event_receiver: Receiver<Event>,
    is_editing: bool,
}

impl DrawEditingLevel {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver = event_manager
            .subscribe(vec![
                Event::NavigatingTo(NavigationScreens::EditLevel).to_string()
            ]);
        let is_editing = false;

        Self {
            event_receiver,
            is_editing,
        }
    }

    pub fn run(
        &mut self,
        context: &mut Context,
        level_manager: &LevelManager,
        world: &World,
    ) -> Result<()> {
        self.update_is_editing()?;
        if !self.is_editing {
            return Ok(());
        }

        let level_name = if let Some(name) = self.get_current_level_name(world)? {
            name
        } else {
            return Ok(());
        };

        let level = level_manager.get_level(&level_name).unwrap();

        let cell_size = self.calculate_cell_size(level, context);
        let mut mesh_builder = MeshBuilder::new();
        let line_height = level.height as f32 * cell_size;

        for count in 0..level.width {
            let start = ggez::mint::Point2 {
                x: count as f32 * cell_size,
                y: 0.0,
            };
            let end = ggez::mint::Point2 {
                x: start.x,
                y: start.y + line_height,
            };
            mesh_builder.line(&[start, end], 1.0, WHITE)?;
        }

        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;

        Ok(())
    }

    fn update_is_editing(&mut self) -> Result<()> {
        if let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::NavigatingTo(NavigationScreens::EditLevel) => self.is_editing = true,
                Event::NavigatingTo(NavigationScreens::Play(_)) => self.is_editing = false,
                _ => {}
            }
        }
        Ok(())
    }

    fn calculate_cell_size(&self, level: &Level, context: &mut Context) -> f32 {
        let screen_coordinates = graphics::screen_coordinates(context);
        let width = screen_coordinates.w / level.width as f32;
        let height = screen_coordinates.h / level.height as f32;

        if width < height {
            width
        } else {
            height
        }
    }

    fn get_current_level_name(&self, world: &World) -> Result<Option<String>> {
        let query;
        let (name_components, selected_components) = query!(
            world,
            query,
            ComponentNames::Name.as_ref(),
            ComponentNames::Selected.as_ref()
        );

        for (index, selected_component) in selected_components.iter().enumerate() {
            let wrapped_selected_component: &DataWrapper<bool> = selected_component.cast()?;
            if *wrapped_selected_component.borrow() {
                let wrapped_name: &DataWrapper<String> = name_components[index].cast()?;
                return Ok(Some(wrapped_name.borrow().clone()));
            }
        }
        Ok(None)
    }
}
