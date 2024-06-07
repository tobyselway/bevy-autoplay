use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

struct InputEvent {
    timestamp: Duration,
    event: ButtonInput<KeyCode>,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum SessionState {
    #[default]
    Stopped,
    Playing,
    Recording,
}

#[derive(Resource, Default)]
struct PlaySession {
    events: VecDeque<InputEvent>,
    start_time: Duration,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(SessionState::Stopped)
        .insert_resource(PlaySession {
            ..Default::default()
        })
        .add_systems(
            Update,
            (
                record.run_if(in_state(SessionState::Recording)),
                playback.run_if(in_state(SessionState::Playing)),
                log_inputs,
            )
                .chain(),
        )
        .add_systems(OnEnter(SessionState::Recording), start_recording)
        .add_systems(OnExit(SessionState::Recording), stop_recording)
        .add_systems(OnEnter(SessionState::Playing), start_playing)
        .add_systems(OnExit(SessionState::Playing), stop_playing)
        .add_systems(Update, toggle_record)
        .add_systems(Update, toggle_play)
        .run();
}

fn start_recording(mut session: ResMut<PlaySession>, time: Res<Time<Virtual>>) {
    session.start_time = time.elapsed();
    info!("Started recording");
}

fn stop_recording() {
    info!("Stopped recording");
}

fn start_playing(mut session: ResMut<PlaySession>, time: Res<Time<Virtual>>) {
    session.start_time = time.elapsed();
    info!("Started playing");
}

fn stop_playing() {
    info!("Stopped playing");
}

fn toggle_record(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    session_state: Res<State<SessionState>>,
    mut next_session_state: ResMut<NextState<SessionState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::F12) {
        return;
    }
    next_session_state.set(match *session_state.get() {
        SessionState::Playing => SessionState::Recording,
        SessionState::Stopped => SessionState::Recording,
        SessionState::Recording => SessionState::Stopped,
    });
}

fn toggle_play(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    session_state: Res<State<SessionState>>,
    mut next_session_state: ResMut<NextState<SessionState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::F11) {
        return;
    }
    next_session_state.set(match *session_state.get() {
        SessionState::Recording => SessionState::Playing,
        SessionState::Stopped => SessionState::Playing,
        SessionState::Playing => SessionState::Stopped,
    });
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

fn record(
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

fn playback(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
    mut next_session_state: ResMut<NextState<SessionState>>,
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
        next_session_state.set(SessionState::Stopped);
    }
}
