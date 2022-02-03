use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::{EventReader, ResMut, MouseButton, EventWriter},
    window::Windows,
};

use crate::{AppState};

pub fn mouse_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>, 
    mut app_state:ResMut<AppState>
) {
    for e in mouse_button_input_events.iter() {
        if e.button == MouseButton::Left && e.state == ElementState::Pressed {
            if app_state.input_locked == false {
                app_state.input_locked = true;
            }
        }
    }
}
