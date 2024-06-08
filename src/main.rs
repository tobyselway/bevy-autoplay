use autoplay::{AutoplayPlugin, AutoplaySet, AutoplayState};
use bevy::prelude::*;

mod autoplay;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AutoplayPlugin))
        .add_systems(Update, toggle_record.before(AutoplaySet))
        .add_systems(Update, toggle_play.before(AutoplaySet))
        .add_systems(Update, log_inputs.after(AutoplaySet))
        .run();
}

fn toggle_record(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    session_state: Res<State<AutoplayState>>,
    mut next_session_state: ResMut<NextState<AutoplayState>>,
) {
    keyboard_input.clear_just_released(KeyCode::F12);
    if !keyboard_input.clear_just_pressed(KeyCode::F12) {
        return;
    }
    next_session_state.set(match *session_state.get() {
        AutoplayState::Playing => AutoplayState::Recording,
        AutoplayState::Stopped => AutoplayState::Recording,
        AutoplayState::Recording => AutoplayState::Stopped,
    });
}

fn toggle_play(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    session_state: Res<State<AutoplayState>>,
    mut next_session_state: ResMut<NextState<AutoplayState>>,
) {
    keyboard_input.clear_just_released(KeyCode::F11);
    if !keyboard_input.clear_just_pressed(KeyCode::F11) {
        return;
    }
    next_session_state.set(match *session_state.get() {
        AutoplayState::Recording => AutoplayState::Playing,
        AutoplayState::Stopped => AutoplayState::Playing,
        AutoplayState::Playing => AutoplayState::Stopped,
    });
}

fn log_inputs(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.get_just_pressed().count() == 0
        && keyboard_input.get_just_released().count() == 0
    {
        return;
    }
    info!("Keyboard input: {:?}", keyboard_input);
}
