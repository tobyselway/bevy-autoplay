use bevy::prelude::*;

use super::{
    session::{InputLog, Session},
    StartTime,
};

pub fn start_recording(
    mut session: ResMut<Session>,
    time: Res<Time<Virtual>>,
    mut start_time: ResMut<StartTime>,
) {
    session.clear();
    start_time.0 = time.elapsed();
    info!("Started recording");
}

pub fn record(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<Session>,
    time: Res<Time<Virtual>>,
    start_time: Res<StartTime>,
) {
    if keyboard_input.get_just_pressed().count() == 0
        && keyboard_input.get_just_released().count() == 0
    {
        return;
    }
    let mut key_event: InputLog<KeyCode> = InputLog::new(time.elapsed() - start_time.0);
    key_event.load_button_input_events(keyboard_input.clone());
    session.push_back(key_event);
}

pub fn stop_recording() {
    info!("Stopped recording");
}
