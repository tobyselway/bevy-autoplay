# bevy-autoplay

![crates.io](https://img.shields.io/crates/v/bevy-autoplay.svg)

Automated integration testing based on recorded play-testing sessions.

## Recording Sessions

```rust
use bevy::{input::InputSystem, prelude::*};
use bevy_autoplay::{AutoplayPlugin, AutoplayState, AutoplaySystem, SaveToFile};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AutoplayPlugin)) // Register the AutoplayPlugin
        .add_systems(
            PreUpdate,
            toggle_record.before(AutoplaySystem).after(InputSystem),
        )
        .add_systems(OnExit(AutoplayState::Recording), after_recording)
        .run();
}

// After recording finished, save the session to a file
fn after_recording(mut ev_save: EventWriter<SaveToFile>) {
    ev_save.send(SaveToFile("tests/sessions/my_session.gsi".into()));
}

// When F12 pressed, start/stop recording
fn toggle_record(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    autoplay_state: Res<State<AutoplayState>>,
    mut next_autoplay_state: ResMut<NextState<AutoplayState>>,
) {
    // Clear in order to avoid the F12 keypress passing through to the recording
    keyboard_input.clear_just_released(KeyCode::F12);
    if !keyboard_input.clear_just_pressed(KeyCode::F12) {
        return;
    }

    // If stopped then record, and vice-versa
    if *autoplay_state.get() == AutoplayState::Stopped {
        next_autoplay_state.set(AutoplayState::Recording);
    } else {
        next_autoplay_state.set(AutoplayState::Stopped);
    }
}
```

## Writing Tests

```rust
use bevy::{
    app::{App, Update},
    prelude::*,
};
use bevy_autoplay::testing::{AutoplayTestPlugin, TestResult};

#[test]
fn player_must_press_f_key() {
    fn f_pressed(mut result: EventWriter<TestResult>, keyboard_input: Res<ButtonInput<KeyCode>>) {
        if keyboard_input.just_pressed(KeyCode::KeyF) { // When F key pressed
            result.send(TestResult::Success); // End test with success
        }
    }

    App::new()
        // Load recorded session
        .add_plugins(AutoplayTestPlugin("tests/sessions/press_f.gsi".into()))
        .add_systems(Update, f_pressed)
        .run();
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
