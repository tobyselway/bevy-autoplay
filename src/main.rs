use std::{collections::VecDeque, error, fs::File, hash::Hash, io::BufReader, time::Duration};

use bevy::{prelude::*, utils::HashSet};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct KeyInput {
    pub pressed: HashSet<KeyCode>,
    pub just_pressed: HashSet<KeyCode>,
    pub just_released: HashSet<KeyCode>,
}

impl From<ButtonInput<KeyCode>> for KeyInput {
    fn from(value: ButtonInput<KeyCode>) -> Self {
        KeyInput {
            pressed: HashSet::from_iter(value.get_pressed().cloned()),
            just_pressed: HashSet::from_iter(value.get_just_pressed().cloned()),
            just_released: HashSet::from_iter(value.get_just_released().cloned()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InputEvent {
    timestamp: Duration,
    event: KeyInput,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum SessionState {
    #[default]
    Stopped,
    Playing,
    Recording,
}

#[derive(Resource)]
struct StartTime(Duration);

#[derive(Resource, Default, Serialize, Deserialize)]
struct PlaySession {
    events: VecDeque<InputEvent>,
}

impl PlaySession {
    fn save(&self, filename: &str) -> Result<(), Box<dyn error::Error>> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))?;
        let mut file = File::create(filename)?;
        file.write_all(&buf)?;
        Ok(())
    }

    fn load(&mut self, filename: &str) -> Result<(), Box<dyn error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        *self = rmp_serde::from_read(reader)?;
        Ok(())
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(SessionState::Stopped)
        .insert_resource(StartTime(Duration::new(0, 0)))
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

fn start_recording(
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
    mut start_time: ResMut<StartTime>,
) {
    session.events.clear();
    start_time.0 = time.elapsed();
    info!("Started recording");
}

fn stop_recording(session: Res<PlaySession>) {
    info!("Stopped recording");
    session.save("recording1.gsi").unwrap();
}

fn start_playing(
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
    mut start_time: ResMut<StartTime>,
) {
    session.load("recording1.gsi").unwrap();
    start_time.0 = time.elapsed();
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
    start_time: Res<StartTime>,
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

    session.events.push_back(InputEvent {
        event: keyboard_input.clone().into(),
        timestamp: time.elapsed() - start_time.0,
    });
}

fn playback(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut session: ResMut<PlaySession>,
    time: Res<Time<Virtual>>,
    mut next_session_state: ResMut<NextState<SessionState>>,
    start_time: Res<StartTime>,
) {
    if let Some(entry) = session.events.front() {
        if time.elapsed() < (entry.timestamp + start_time.0) {
            return;
        }
        for pressed in entry.event.just_pressed.iter() {
            keyboard_input.press(*pressed);
        }
        for released in entry.event.just_released.iter() {
            keyboard_input.release(*released);
        }
        session.events.pop_front();
    } else {
        next_session_state.set(SessionState::Stopped);
    }
}
