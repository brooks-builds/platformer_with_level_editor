use bbecs::world::World;
use crossbeam::channel::{SendError, Sender};
use eyre::Result;

use crate::{
    events::{event::Event, EventManager},
    helpers::{query_end, query_player::query_player},
};

pub struct CollideWithEnd {
    event_sender: Sender<Event>,
}

impl CollideWithEnd {
    pub fn new(event_manager: &EventManager) -> Self {
        let event_sender = event_manager.register();

        Self { event_sender }
    }

    pub fn run(&mut self, world: &World) -> Result<()> {
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };
        let end = if let Some(end) = query_end::query_end(world)? {
            end
        } else {
            return Ok(());
        };

        if player.is_colliding_with(&end) {
            dbg!("colliding with end");
        }
        Ok(())
    }
}
