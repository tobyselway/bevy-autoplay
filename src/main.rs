use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

struct InputEvent {
    timestamp: Duration,
    event: ButtonInput<KeyCode>,
}

#[derive(Resource)]
struct Recording {
    events: VecDeque<InputEvent>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Recording {
            events: VecDeque::new(),
        })
        .add_systems(Update, record_inputs)
        .add_systems(Update, playback_inputs)
        .add_systems(Update, log_inputs)
        .run();
}

fn log_inputs(keyboard_input: Res<ButtonInput<KeyCode>>) {
    let just_pressed = keyboard_input.get_just_pressed();
    let just_released = keyboard_input.get_just_released();

    if just_pressed.count() == 0 && just_released.count() == 0 {
        return;
    }

    info!("Keyboard input: {:?}", keyboard_input);
}

fn record_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut recording: ResMut<Recording>,
    time: Res<Time>,
) {
    let just_pressed = keyboard_input.get_just_pressed();
    let just_released = keyboard_input.get_just_released();

    if just_pressed.count() == 0 && just_released.count() == 0 {
        return;
    }

    recording.events.push_back(InputEvent {
        event: keyboard_input.clone(),
        timestamp: time.elapsed(),
    });
}

fn playback_inputs(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut recording: ResMut<Recording>,
    time: Res<Time>,
) {
    let t = time
        .elapsed()
        .checked_sub(Duration::from_secs(5)) //* Replay with a 5s delay
        .unwrap_or(Duration::from_micros(0));

    if let Some(entry) = recording.events.front() {
        if entry.timestamp > t {
            return;
        }
        for pressed in entry.event.get_just_pressed() {
            keyboard_input.press(*pressed)
        }
        for released in entry.event.get_just_released() {
            keyboard_input.release(*released)
        }
        recording.events.pop_front();
    }
}
