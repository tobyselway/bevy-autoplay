use bevy::{
    app::{App, AppExit, Plugin, Startup},
    input::InputPlugin,
    prelude::*,
    MinimalPlugins,
};

use super::{AutoplayPlugin, LoadFromFileAndPlay};

#[derive(Resource)]
struct TestSessionFilename(String);

#[derive(Event)]
pub enum TestResult {
    #[allow(dead_code)]
    Success,
    #[allow(dead_code)]
    Failure(String),
}

pub struct AutoplayTestPlugin(pub String);

impl Plugin for AutoplayTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MinimalPlugins, InputPlugin, AutoplayPlugin))
            .insert_resource(TestSessionFilename(self.0.clone()))
            .add_event::<TestResult>()
            .add_systems(Startup, playback_recording)
            .add_systems(Update, check_for_result);
    }
}

fn playback_recording(
    mut ev_load_play: EventWriter<LoadFromFileAndPlay>,
    filename: Res<TestSessionFilename>,
    mut _time: ResMut<Time<Virtual>>,
) {
    // time.set_relative_speed(10.0); // TODO: Make this configurable
    ev_load_play.send(LoadFromFileAndPlay(filename.0.clone()));
}

fn check_for_result(mut exit: EventWriter<AppExit>, mut ev_result: EventReader<TestResult>) {
    if let Some(ev) = ev_result.read().next() {
        match ev {
            TestResult::Success => exit.send(AppExit),
            TestResult::Failure(msg) => panic!("{}", msg),
        };
    }
}
