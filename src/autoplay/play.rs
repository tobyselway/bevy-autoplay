use bevy::prelude::*;

use super::{
    session::{ButtonEvent, Session},
    storable::Storable,
    AutoplayState, StartTime,
};

pub fn start_playing(
    mut session: ResMut<Session>,
    time: Res<Time<Virtual>>,
    mut start_time: ResMut<StartTime>,
) {
    session.load("recording1.gsi").unwrap();
    start_time.0 = time.elapsed();
    info!("Started playing");
}

pub fn play(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut session: ResMut<Session>,
    time: Res<Time<Virtual>>,
    mut next_session_state: ResMut<NextState<AutoplayState>>,
    start_time: Res<StartTime>,
) {
    if let Some(entry) = session.front() {
        if time.elapsed() < (entry.timestamp() + start_time.0) {
            return;
        }
        for event in entry.events().iter() {
            match *event {
                ButtonEvent::Press(key) => keyboard_input.press(key),
                ButtonEvent::Release(key) => keyboard_input.release(key),
            }
        }
        session.pop_front();
    } else {
        next_session_state.set(AutoplayState::Stopped);
    }
}

pub fn stop_playing() {
    info!("Stopped playing");
}
