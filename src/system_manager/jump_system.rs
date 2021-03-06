use bbecs::{data_types::point::Point, world::World};
use crossbeam::channel::Receiver;
use eyre::Result;

use crate::{
    command::Command,
    events::{event::Event, EventManager},
    helpers::{self, get_resource::get_f32},
    names::{entity_states::EntityStates, resource_names::ResourceNames},
};

pub struct JumpSystem {
    event_receiver: Receiver<Event>,
}

impl JumpSystem {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver =
            event_manager.subscribe(vec![Event::Command(Command::Jump).to_string()]);

        Self { event_receiver }
    }

    pub fn run(&mut self, world: &World) -> Result<()> {
        if !self.did_jump()? {
            return Ok(());
        }
        let mut player = if let Some(player) = helpers::query_player::query_player(world)? {
            player
        } else {
            return Ok(());
        };
        if player.state().unwrap() != EntityStates::Standing {
            return Ok(());
        }

        let jump_force = Point::new(0.0, -get_f32(world, ResourceNames::JumpForce.as_ref()));

        *player.acceleration.unwrap().borrow_mut() += jump_force;
        player.set_state(EntityStates::Falling);
        Ok(())
    }

    fn did_jump(&mut self) -> Result<bool> {
        Ok(matches!(
            self.event_receiver.try_recv(),
            Ok(Event::Command(Command::Jump))
        ))
    }
}
