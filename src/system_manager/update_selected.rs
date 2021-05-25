use bbecs::components::{CastComponents, ComponentData};
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use crossbeam::channel::{Receiver, Sender};
use eyre::{bail, Result};

use crate::command::Command;
use crate::events::event::Event;
use crate::events::EventManager;
use crate::names::component_names::ComponentNames;

pub struct UpdateSelectedSystem {
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

impl UpdateSelectedSystem {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_receiver =
            event_manager.subscribe(vec![Event::Command(Command::SelectUp).to_string()]);
        let event_sender = event_manager.register();

        Self {
            event_receiver,
            event_sender,
        }
    }

    pub fn run(&self, world: &World) -> Result<()> {
        let current_command = if let Some(command) = self.get_command() {
            command
        } else {
            return Ok(());
        };

        let query;
        let (selected_components, _selectable_components) = query!(
            world,
            query,
            ComponentNames::Selected.as_ref(),
            ComponentNames::Selectable.as_ref()
        );
        let current_index = self.get_selected_index(selected_components)?;

        self.deselect_selected_component(current_index, selected_components)?;
        let new_index = self.calculate_selected_index(
            current_index,
            selected_components.len(),
            current_command,
        );

        if new_index != current_index {
            self.mark_component_as_selected(new_index, selected_components)?;
            self.event_sender.send(Event::ChangeMenuItem)?;
        }

        Ok(())
    }

    fn get_command(&self) -> Option<Command> {
        let mut result = None;
        while let Ok(event) = self.event_receiver.try_recv() {
            result = match event {
                Event::Command(command) => match command {
                    Command::SelectUp => Some(command),
                    Command::SelectDown => Some(command),
                },
                _ => None,
            }
        }
        result
    }

    fn get_selected_index(&self, selected_components: &[&ComponentData]) -> Result<usize> {
        for (index, selected_component) in selected_components.iter().enumerate() {
            let wrapped_selected: &DataWrapper<bool> = selected_component.cast()?;
            if *wrapped_selected.borrow() {
                return Ok(index);
            }
        }

        bail!("Could not find selected index of menu item")
    }

    fn deselect_selected_component(
        &self,
        index: usize,
        selected_components: &[&ComponentData],
    ) -> Result<()> {
        let wrapped_selected: &DataWrapper<bool> = selected_components[index].cast()?;
        *wrapped_selected.borrow_mut() = false;
        Ok(())
    }

    fn calculate_selected_index(
        &self,
        current_index: usize,
        selected_components_length: usize,
        command: Command,
    ) -> usize {
        match command {
            Command::SelectUp => {
                if current_index == 0 {
                    selected_components_length - 1
                } else {
                    current_index - 1
                }
            }
            Command::SelectDown => {
                if current_index == selected_components_length - 1 {
                    0
                } else {
                    current_index + 1
                }
            }
        }
    }

    fn mark_component_as_selected(
        &self,
        index: usize,
        selected_components: &[&ComponentData],
    ) -> Result<()> {
        let wrapped_selected: &DataWrapper<bool> = selected_components[index].cast()?;
        *wrapped_selected.borrow_mut() = true;
        Ok(())
    }
}
