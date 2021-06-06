use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use crate::events::EventManager;
use crate::image_manager::ImageManager;
use crate::level_manager::LevelManager;

use self::apply_friction_system::ApplyFrictionSystem;
use self::apply_gravity::ApplyGravitySystem;
use self::camera::CameraSystem;
use self::collide_with_end::CollideWithEnd;
use self::collide_with_platform::CollideWithPlatform;
use self::display_collision_border_system::DisplayCollisionBorderSystem;
use self::draw_editing_level::DrawEditingLevel;
use self::draw_selectable_text::DrawText;
use self::editing_level_system::EditingLevelSystem;
use self::jump_system::JumpSystem;
use self::move_player::MovePlayer;
use self::update_camera_position::UpdateCameraPosition;
use self::update_forces::UpdateForcesSystem;
use self::update_selected::UpdateSelectedSystem;
use self::update_text::UpdateTextSystem;

mod apply_friction_system;
mod apply_gravity;
mod camera;
mod collide_with_end;
mod collide_with_platform;
mod display_collision_border_system;
mod draw_editing_level;
mod draw_selectable_text;
mod editing_level_system;
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
    apply_friction_system: ApplyFrictionSystem,
    editing_level_system: EditingLevelSystem,
    display_collision_border_system: DisplayCollisionBorderSystem,
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
        let apply_friction_system = ApplyFrictionSystem;
        let editing_level_system = EditingLevelSystem::new(event_manager);
        let display_collision_border_system = DisplayCollisionBorderSystem;

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
            apply_friction_system,
            editing_level_system,
            display_collision_border_system,
        }
    }

    pub fn update(
        &mut self,
        world: &World,
        context: &mut Context,
        level_manager: &LevelManager,
    ) -> Result<()> {
        self.update_text.run(world, context)?;
        self.update_selected.run(world)?;
        self.apply_gravity.run(world)?;
        self.update_forces.run(world)?;
        self.collide_with_platform.run(world)?;
        self.update_camera_position.run(world)?;
        self.move_player.run(world)?;
        self.jump_system.run(world)?;
        self.collide_with_end.run(world)?;
        self.apply_friction_system.run(world)?;
        self.draw_editing_level.update()?;
        self.editing_level_system.run(level_manager, context)?;
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
        self.display_collision_border_system.run(world, context)?;

        Ok(())
    }
}
