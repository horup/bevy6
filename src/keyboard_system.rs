use bevy::{prelude::{EventReader, EventWriter, KeyCode}, input::keyboard::KeyboardInput};

use crate::AppEvent;

pub fn keyboard_system(mut keyboard_input_events: EventReader<KeyboardInput>, mut app_event_writer: EventWriter<AppEvent>) {
    for e in keyboard_input_events.iter() {
        if let Some(key_code) = e.key_code {
            if key_code == KeyCode::Escape {
                app_event_writer.send(AppEvent::LockInput(false));
            }
        }
    }    
}