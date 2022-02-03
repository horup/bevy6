use bevy::{prelude::{EventReader, EventWriter, KeyCode, ResMut}, input::keyboard::KeyboardInput};

use crate::{AppState};

pub fn keyboard_system(mut keyboard_input_events: EventReader<KeyboardInput>, mut app_state:ResMut<AppState>) {
    for e in keyboard_input_events.iter() {
        if let Some(key_code) = e.key_code {
            if key_code == KeyCode::Escape {
                if app_state.input_locked {
                    app_state.input_locked = false;
                }
            }
        }
    }    
}