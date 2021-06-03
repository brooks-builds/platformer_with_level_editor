use bbecs::{
    data_types::point::Point,
    world::{World, WorldMethods},
};
use eyre::Result;
use ggez::graphics::{Text, TextFragment};

use crate::names::{component_names::ComponentNames, entity_states::EntityStates};

#[derive(Default)]
pub struct InsertIntoWorld {
    navigate_to: Option<String>,
    position: Option<Point>,
    selectable: Option<bool>,
    selected: Option<bool>,
    text: Option<Text>,
    text_fragment: Option<TextFragment>,
    velocity: Option<Point>,
    acceleration: Option<Point>,
    affected_by_gravity: Option<bool>,
    camera: Option<bool>,
    width: Option<f32>,
    height: Option<f32>,
    platform: Option<bool>,
    image_name: Option<String>,
    player: Option<bool>,
    name: Option<String>,
    state: Option<EntityStates>,
}

impl InsertIntoWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_navigate_to(mut self, navigate_to: String) -> Self {
        self.navigate_to = Some(navigate_to);
        self
    }

    pub fn set_position(mut self, position: Point) -> Self {
        self.position = Some(position);
        self
    }

    pub fn set_selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    pub fn set_selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn set_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    pub fn set_text_fragment(mut self, text_fragment: TextFragment) -> Self {
        self.text_fragment = Some(text_fragment);
        self
    }

    pub fn set_velocity(mut self, velocity: Point) -> Self {
        self.velocity = Some(velocity);
        self
    }

    pub fn set_acceleration(mut self, acceleration: Point) -> Self {
        self.acceleration = Some(acceleration);
        self
    }

    pub fn set_affected_by_gravity(mut self, affected_by_gravity: bool) -> Self {
        self.affected_by_gravity = Some(affected_by_gravity);
        self
    }

    pub fn set_camera(mut self, camera: bool) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn set_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn set_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn set_platform(mut self) -> Self {
        self.platform = Some(true);
        self
    }

    pub fn set_image_name(mut self, image_name: String) -> Self {
        self.image_name = Some(image_name);
        self
    }

    pub fn set_player(mut self) -> Self {
        self.player = Some(true);
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn set_state(mut self, state: EntityStates) -> Self {
        self.state = Some(state);
        self
    }

    pub fn insert(self, world: &mut World) -> Result<()> {
        world.spawn_entity()?;

        if let Some(navigate_to) = self.navigate_to {
            world.with_component(ComponentNames::NavigateTo.as_ref(), navigate_to)?;
        }

        if let Some(position) = self.position {
            world.with_component(ComponentNames::Position.as_ref(), position)?;
        }

        if let Some(selectable) = self.selectable {
            world.with_component(ComponentNames::Selectable.as_ref(), selectable)?;
        }

        if let Some(selected) = self.selected {
            world.with_component(ComponentNames::Selected.as_ref(), selected)?;
        }

        if let Some(text) = self.text {
            world.with_component(ComponentNames::Text.as_ref(), text)?;
        }

        if let Some(text_fragment) = self.text_fragment {
            world.with_component(ComponentNames::TextFragment.as_ref(), text_fragment)?;
        }

        if let Some(velocity) = self.velocity {
            world.with_component(ComponentNames::Velocity.as_ref(), velocity)?;
        }

        if let Some(acceleration) = self.acceleration {
            world.with_component(ComponentNames::Acceleration.as_ref(), acceleration)?;
        }

        if let Some(affected_by_gravity) = self.affected_by_gravity {
            world.with_component(
                ComponentNames::AffectedByGravity.as_ref(),
                affected_by_gravity,
            )?;
        }

        if let Some(camera) = self.camera {
            world.with_component(ComponentNames::Camera.as_ref(), camera)?;
        }

        if let Some(width) = self.width {
            world.with_component(ComponentNames::Width.as_ref(), width)?;
        }

        if let Some(height) = self.height {
            world.with_component(ComponentNames::Height.as_ref(), height)?;
        }

        if let Some(platform) = self.platform {
            world.with_component(ComponentNames::Platform.as_ref(), platform)?;
        }

        if let Some(image_name) = self.image_name {
            world.with_component(ComponentNames::ImageName.as_ref(), image_name)?;
        }

        if let Some(player) = self.player {
            world.with_component(ComponentNames::Player.as_ref(), player)?;
        }

        if let Some(name) = self.name {
            world.with_component(ComponentNames::Name.as_ref(), name)?;
        }

        if let Some(state) = self.state {
            world.with_component(ComponentNames::EntityState.as_ref(), state.to_string())?;
        }

        Ok(())
    }
}
