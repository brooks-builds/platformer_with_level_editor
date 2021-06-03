use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use crate::events::EventManager;
use crate::image_manager::ImageManager;
use crate::level_manager::LevelManager;

use self::apply_gravity::ApplyGravitySystem;
use self::camera::CameraSystem;
use self::collide_with_end::CollideWithEnd;
use self::collide_with_platform::CollideWithPlatform;
use self::draw_editing_level::DrawEditingLevel;
use self::draw_selectable_text::DrawText;
use self::jump_system::JumpSystem;
use self::move_player::MovePlayer;
use self::update_camera_position::UpdateCameraPosition;
use self::update_forces::UpdateForcesSystem;
use self::update_selected::UpdateSelectedSystem;
use self::update_text::UpdateTextSystem;

mod apply_gravity;
mod camera;
mod collide_with_end;
mod collide_with_platform;
mod draw_editing_level;
mod draw_selectable_text;
mod jump_system;
mod move_player;
mod update_camera_position;
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
    collide_with_platform: CollideWithPlatform,
    update_camera_position: UpdateCameraPosition,
    move_player: MovePlayer,
    draw_editing_level: DrawEditingLevel,
    jump_system: JumpSystem,
    collide_with_end: CollideWithEnd,
}

impl SystemManager {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let draw_text = DrawText::default();
        let update_text = UpdateTextSystem::default();
        let update_selected = UpdateSelectedSystem::new(event_manager);
        let draw_entities = CameraSystem;
        let apply_gravity = ApplyGravitySystem;
        let update_forces = UpdateForcesSystem;
        let collide_with_platform = CollideWithPlatform;
        let update_camera_position = UpdateCameraPosition;
        let move_player = MovePlayer::new(event_manager);
        let draw_editing_level = DrawEditingLevel::new(event_manager);
        let jump_system = JumpSystem::new(event_manager);
        let collide_with_end = CollideWithEnd::new(&event_manager);

        Self {
            draw_text,
            update_text,
            update_selected,
            draw_entities,
            apply_gravity,
            update_forces,
            collide_with_platform,
            update_camera_position,
            move_player,
            draw_editing_level,
            jump_system,
            collide_with_end,
        }
    }

    pub fn update(&mut self, world: &World, context: &mut Context) -> Result<()> {
        self.update_text.run(world, context)?;
        self.update_selected.run(world)?;
        self.apply_gravity.run(world)?;
        self.update_forces.run(world)?;
        self.collide_with_platform.run(world)?;
        self.update_camera_position.run(world)?;
        self.move_player.run(world)?;
        self.jump_system.run(world)?;
        self.collide_with_end.run(world)?;
        Ok(())
    }

    pub fn display(
        &mut self,
        world: &World,
        context: &mut Context,
        image_manager: &ImageManager,
        level_manager: &LevelManager,
    ) -> Result<()> {
        self.draw_text.run(world, context)?;
        self.draw_entities.run(world, context, image_manager)?;
        self.draw_editing_level.run(context, level_manager, world)?;

        Ok(())
    }
}
