use bbecs::{data_types::point::Point, world::World};
use crossbeam::channel::Receiver;
use eyre::Result;

use crate::{
    command::Command,
    events::{event::Event, EventManager},
    helpers::{get_resource::get_f32, query_player::query_player},
    names::resource_names::ResourceNames,
};

pub struct MovePlayer {
    is_moving_right: bool,
    is_moving_left: bool,
    event_receiver: Receiver<Event>,
    is_playing: bool,
}

impl MovePlayer {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let is_moving_right = false;
        let is_moving_left = false;
        let event_receiver = event_manager.subscribe(vec![
            Event::Command(Command::StartMovingLeft).to_string(),
            Event::Won.to_string(),
        ]);
        let is_playing = true;

        Self {
            is_moving_right,
            is_moving_left,
            event_receiver,
            is_playing,
        }
    }

    pub fn run(&mut self, world: &World) -> Result<()> {
        if !self.is_playing {
            return Ok(());
        }
        let player = if let Some(player) = query_player(world)? {
            player
        } else {
            return Ok(());
        };

        self.handle_events()?;

        if self.is_not_moving() {
            return Ok(());
        }

        let move_force = self.create_move_force(world);
        *player.acceleration.unwrap().borrow_mut() += move_force;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::Command(Command::StartMovingLeft) => self.is_moving_left = true,
                Event::Command(Command::StartMovingRight) => self.is_moving_right = true,
                Event::Command(Command::StopMovingLeft) => self.is_moving_left = false,
                Event::Command(Command::StopMovingRight) => self.is_moving_right = false,
                Event::Won => self.is_playing = false,
                _ => {}
            }
        }

        Ok(())
    }

    fn is_not_moving(&self) -> bool {
        !self.is_moving_left && !self.is_moving_right
    }

    fn create_move_force(&self, world: &World) -> Point {
        let mut force = Point::new(0.0, 0.0);
        let move_speed = get_f32(world, ResourceNames::PlayerMoveSpeed.as_ref());
        force.x = if self.is_moving_right {
            move_speed
        } else {
            -move_speed
        };
        force
    }
}
