use std::{collections::VecDeque, hash::Hash, time::Duration};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ButtonEvent<T> {
    Press(T),
    Release(T),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputLog<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    timestamp: Duration,
    events: Vec<ButtonEvent<T>>,
}

impl<T> InputLog<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    pub fn new(timestamp: Duration) -> Self {
        Self {
            timestamp,
            events: Vec::new(),
        }
    }

    pub fn load_button_input_events(&mut self, button_input: ButtonInput<T>) {
        button_input.get_just_pressed().for_each(|item| {
            self.events.push(ButtonEvent::Press(*item));
        });
        button_input.get_just_released().for_each(|item| {
            self.events.push(ButtonEvent::Release(*item));
        });
    }

    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    pub fn events(&self) -> &Vec<ButtonEvent<T>> {
        &self.events
    }
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Session {
    key_inputs: VecDeque<InputLog<KeyCode>>,
}

impl Session {
    pub fn clear(&mut self) {
        self.key_inputs.clear();
    }

    pub fn push_back(&mut self, event: InputLog<KeyCode>) {
        self.key_inputs.push_back(event);
    }

    pub fn pop_front(&mut self) -> Option<InputLog<KeyCode>> {
        self.key_inputs.pop_front()
    }

    pub fn front(&self) -> Option<&InputLog<KeyCode>> {
        self.key_inputs.front()
    }
}
