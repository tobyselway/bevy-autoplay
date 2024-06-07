use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

struct InputEvent {
    timestamp: Duration,
    event: ButtonInput<KeyCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
enum SessionMode {
    #[default]
    Stop,
    Play,
    Record,
}

#[derive(Resource, Default)]
struct PlaySession {
    mode: SessionMode,
    events: VecDeque<InputEvent>,
    start_time: Duration,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PlaySession {
            mode: SessionMode::Stop,
            ..Default::default()
        })
        .add_systems(
            Update,
            (
                record_inputs.run_if(session_is_recording),
                playback_inputs.run_if(session_is_playing),
                log_inputs,
            )
                .chain(),
        )
        .add_systems(Update, toggle_record)
        .add_systems(Update, toggle_play)
        .run();
}

fn session_is_playing(session: Res<PlaySession>) -> bool {
    session.mode == SessionMode::Play
}

fn session_is_recording(session: Res<PlaySession>) -> bool {
    session.mode == SessionMode::Record
}

fn toggle_record(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
) {
    if !keyboard_input.just_pressed(KeyCode::F12) {
        return;
    }
    session.mode = match session.mode {
        SessionMode::Stop => {
            session.start_time = time.elapsed();
            info!("Started recording");
            SessionMode::Record
        }
        SessionMode::Record => {
            info!("Stopped recording");
            SessionMode::Stop
        }
        SessionMode::Play => {
            info!("Stopped playing");
            session.start_time = time.elapsed();
            info!("Started recording");
            SessionMode::Record
        }
    };
}

fn toggle_play(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
) {
    if !keyboard_input.just_pressed(KeyCode::F11) {
        return;
    }
    session.mode = match session.mode {
        SessionMode::Stop => {
            session.start_time = time.elapsed();
            info!("Started playing");
            SessionMode::Play
        }
        SessionMode::Record => {
            info!("Stopped recording");
            session.start_time = time.elapsed();
            info!("Started playing");
            SessionMode::Play
        }
        SessionMode::Play => {
            info!("Stopped playing");
            SessionMode::Stop
        }
    };
}

fn log_inputs(keyboard_input: Res<ButtonInput<KeyCode>>) {
    let just_pressed = keyboard_input.get_just_pressed();
    let just_released = keyboard_input.get_just_released();

    if just_pressed.count() == 0 && just_released.count() == 0 {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::F12) || keyboard_input.just_released(KeyCode::F12) {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::F11) || keyboard_input.just_released(KeyCode::F11) {
        return;
    }

    info!("Keyboard input: {:?}", keyboard_input);
}

fn record_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
) {
    let just_pressed = keyboard_input.get_just_pressed();
    let just_released = keyboard_input.get_just_released();

    if just_pressed.count() == 0 && just_released.count() == 0 {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::F12) || keyboard_input.just_released(KeyCode::F12) {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::F11) || keyboard_input.just_released(KeyCode::F11) {
        return;
    }

    let start_time = session.start_time;
    session.events.push_back(InputEvent {
        event: keyboard_input.clone(),
        timestamp: time.elapsed() - start_time,
    });
}

fn playback_inputs(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
) {
    if let Some(entry) = session.events.front() {
        if time.elapsed() < (entry.timestamp + session.start_time) {
            return;
        }
        for pressed in entry.event.get_just_pressed() {
            keyboard_input.press(*pressed);
        }
        for released in entry.event.get_just_released() {
            keyboard_input.release(*released);
        }
        session.events.pop_front();
    } else {
        session.mode = SessionMode::Stop;
        info!("Stopped playing");
    }
}
