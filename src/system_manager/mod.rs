use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use crate::events::EventManager;

use self::apply_gravity::ApplyGravitySystem;
use self::draw_entities::DrawEntities;
use self::draw_selectable_text::DrawText;
use self::update_selected::UpdateSelectedSystem;
use self::update_text::UpdateTextSystem;

mod apply_gravity;
mod draw_entities;
mod draw_selectable_text;
mod update_selected;
mod update_text;

pub struct SystemManager {
    draw_text: DrawText,
    update_text: UpdateTextSystem,
    update_selected: UpdateSelectedSystem,
    draw_entities: DrawEntities,
    apply_gravity: ApplyGravitySystem,
}

impl SystemManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let draw_text = DrawText::default();
        let update_text = UpdateTextSystem::default();
        let update_selected = UpdateSelectedSystem::new(event_manager);
        let draw_entities = DrawEntities;
        let apply_gravity = ApplyGravitySystem;

        Self {
            draw_text,
            update_text,
            update_selected,
            draw_entities,
            apply_gravity,
        }
    }

    pub fn update(&self, world: &World, context: &mut Context) -> Result<()> {
        self.update_text.run(world, context)?;
        self.update_selected.run(world)?;
        self.apply_gravity.run(world)?;
        Ok(())
    }

    pub fn display(&self, world: &World, context: &mut Context) -> Result<()> {
        self.draw_text.run(world, context)?;
        self.draw_entities.run(world, context)?;
        Ok(())
    }
}
