use bevy::{
    input::{keyboard::KeyboardInput, Input},
    math::Vec2,
    prelude::{EventReader, EventWriter, KeyCode, Res, ResMut},
};

use crate::AppState;

pub fn keyboard_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_state: ResMut<AppState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for e in keyboard_input_events.iter() {
        if let Some(key_code) = e.key_code {
            if key_code == KeyCode::Escape {
                if app_state.input_locked {
                    app_state.input_locked = false;
                }
            }
        }

        app_state.dpad = Vec2::default();
        if app_state.input_locked {
            let mut v = Vec2::default();
            if keyboard_input.pressed(KeyCode::A) {
                v.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                v.x += 1.0;
            }

            if keyboard_input.pressed(KeyCode::W) {
                v.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                v.y -= 1.0;
            }

            app_state.dpad = v;
        }
    }
}
