use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
};
use play::{play, start_playing, stop_playing};
use record::{record, start_recording, stop_recording};
use save::{load_recording, load_recording_and_play, save_recording};
use session::Session;

mod play;
mod record;
mod save;
mod session;
mod storable;
pub mod testing;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AutoplayState {
    #[default]
    Stopped,
    Playing,
    Recording,
}

#[derive(Resource)]
struct StartTime(pub Duration);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutoplaySet;

#[derive(Event)]
pub struct SaveToFile(pub String);

#[derive(Event)]
pub struct LoadFromFile(pub String);

#[derive(Event)]
pub struct LoadFromFileAndPlay(pub String);

pub struct AutoplayPlugin;

impl Plugin for AutoplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AutoplayState::Stopped)
            .insert_resource(StartTime(Duration::new(0, 0)))
            .insert_resource(Session::default())
            .add_event::<SaveToFile>()
            .add_event::<LoadFromFile>()
            .add_event::<LoadFromFileAndPlay>()
            .add_systems(OnEnter(AutoplayState::Recording), start_recording)
            .add_systems(OnExit(AutoplayState::Recording), stop_recording)
            .add_systems(OnEnter(AutoplayState::Playing), start_playing)
            .add_systems(OnExit(AutoplayState::Playing), stop_playing)
            .add_systems(
                Update,
                (save_recording, load_recording, load_recording_and_play),
            )
            .add_systems(
                Update,
                (
                    record.run_if(in_state(AutoplayState::Recording)),
                    play.run_if(in_state(AutoplayState::Playing)),
                )
                    .chain()
                    .in_set(AutoplaySet),
            );
    }
}
