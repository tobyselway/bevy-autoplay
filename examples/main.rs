use bevy::{input::InputSystem, prelude::*};
use bevy_autoplay::{
    AutoplayPlugin, AutoplayState, AutoplaySystem, LoadFromFileAndPlay, SaveToFile,
};
use chrono::Utc;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AutoplayPlugin))
        .add_systems(
            PreUpdate,
            (toggle_record, toggle_play)
                .before(AutoplaySystem)
                .after(InputSystem),
        )
        .add_systems(Update, log_inputs)
        .add_systems(OnExit(AutoplayState::Recording), after_recording)
        .run();
}

fn after_recording(mut ev_save: EventWriter<SaveToFile>) {
    ev_save.send(SaveToFile(format!(
        "examples/sessions/{}.gsi",
        Utc::now().timestamp_millis()
    )));
}

fn toggle_record(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    autoplay_state: Res<State<AutoplayState>>,
    mut next_autoplay_state: ResMut<NextState<AutoplayState>>,
) {
    keyboard_input.clear_just_released(KeyCode::F12);
    if !keyboard_input.clear_just_pressed(KeyCode::F12) {
        return;
    }
    next_autoplay_state.set(match *autoplay_state.get() {
        AutoplayState::Playing => AutoplayState::Recording,
        AutoplayState::Stopped => AutoplayState::Recording,
        AutoplayState::Recording => AutoplayState::Stopped,
    });
}

fn toggle_play(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    autoplay_state: Res<State<AutoplayState>>,
    mut next_autoplay_state: ResMut<NextState<AutoplayState>>,
    mut ev_load_play: EventWriter<LoadFromFileAndPlay>,
) {
    keyboard_input.clear_just_released(KeyCode::F11);
    if !keyboard_input.clear_just_pressed(KeyCode::F11) {
        return;
    }
    if *autoplay_state.get() == AutoplayState::Playing {
        next_autoplay_state.set(AutoplayState::Stopped);
        return;
    }
    ev_load_play.send(LoadFromFileAndPlay(
        "examples/sessions/1717878890687.gsi".into(),
    ));
}

fn log_inputs(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.get_just_pressed().count() == 0
        && keyboard_input.get_just_released().count() == 0
    {
        return;
    }
    info!("Keyboard input: {:?}", keyboard_input);
}

#[cfg(test)]
mod tests {
    use bevy::{
        app::{App, Update},
        prelude::*,
    };
    use bevy_autoplay::testing::{AutoplayTestPlugin, TestResult};

    #[test]
    fn player_must_press_f_key() {
        fn f_pressed(
            mut result: EventWriter<TestResult>,
            keyboard_input: Res<ButtonInput<KeyCode>>,
        ) {
            if keyboard_input.just_pressed(KeyCode::KeyF) {
                result.send(TestResult::Success);
            }
        }

        App::new()
            .add_plugins(AutoplayTestPlugin(
                "examples/sessions/1717878890687.gsi".into(),
            ))
            .add_systems(Update, f_pressed)
            .run();
    }
}
