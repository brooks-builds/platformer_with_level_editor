use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use crate::events::EventManager;
use crate::image_manager::ImageManager;

use self::apply_gravity::ApplyGravitySystem;
use self::camera::CameraSystem;
use self::draw_selectable_text::DrawText;
use self::update_forces::UpdateForcesSystem;
use self::update_selected::UpdateSelectedSystem;
use self::update_text::UpdateTextSystem;

mod apply_gravity;
mod camera;
mod draw_selectable_text;
mod update_forces;
mod update_selected;
mod update_text;

pub struct SystemManager {
    draw_text: DrawText,
    update_text: UpdateTextSystem,
    update_selected: UpdateSelectedSystem,
    draw_entities: CameraSystem,
    apply_gravity: ApplyGravitySystem,
    update_forces: UpdateForcesSystem,
}

impl SystemManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let draw_text = DrawText::default();
        let update_text = UpdateTextSystem::default();
        let update_selected = UpdateSelectedSystem::new(event_manager);
        let draw_entities = CameraSystem;
        let apply_gravity = ApplyGravitySystem;
        let update_forces = UpdateForcesSystem;

        Self {
            draw_text,
            update_text,
            update_selected,
            draw_entities,
            apply_gravity,
            update_forces,
        }
    }

    pub fn update(&self, world: &World, context: &mut Context) -> Result<()> {
        self.update_text.run(world, context)?;
        self.update_selected.run(world)?;
        self.apply_gravity.run(world)?;
        self.update_forces.run(world)?;
        Ok(())
    }

    pub fn display(
        &self,
        world: &World,
        context: &mut Context,
        image_manager: &ImageManager,
    ) -> Result<()> {
        self.draw_text.run(world, context)?;
        self.draw_entities.run(world, context, image_manager)?;

        Ok(())
    }
}
