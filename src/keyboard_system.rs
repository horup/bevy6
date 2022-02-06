use bevy::{
    input::{keyboard::KeyboardInput, Input},
    math::Vec2,
    prelude::{EventReader, EventWriter, KeyCode, Res, ResMut},
};

use crate::{Global, Command};

pub fn keyboard_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_state: ResMut<Global>,
    keyboard_input: Res<Input<KeyCode>>,
    mut command_writer:EventWriter<Command>
) {
    for e in keyboard_input_events.iter() {
        if let Some(key_code) = e.key_code {
            if key_code == KeyCode::Escape {
                if app_state.input_locked {
                    app_state.input_locked = false;
                }
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

        let quick_save_path = "quick.sav".into();
        if keyboard_input.just_pressed(KeyCode::F5) {
            command_writer.send(Command::SaveWorld {
                path:quick_save_path
            })
        } else if keyboard_input.just_pressed(KeyCode::F9) {
            command_writer.send(Command::LoadWorld {
                path:quick_save_path
            });
        }
    }
    
}
