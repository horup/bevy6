use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::{EventReader, ResMut, MouseButton, EventWriter},
    window::Windows,
};

use crate::AppEvent;

pub fn mouse_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>, 
    mut app_event_writer: EventWriter<AppEvent>
) {
    for e in mouse_button_input_events.iter() {
        if e.button == MouseButton::Left && e.state == ElementState::Pressed {
           app_event_writer.send(AppEvent::LockInput(true));
        }
    }
}
