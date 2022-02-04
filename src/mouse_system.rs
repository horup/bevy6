use bevy::{
    input::{mouse::{MouseButtonInput, MouseMotion}, ElementState},
    prelude::{EventReader, ResMut, MouseButton, EventWriter},
    window::Windows, math::Vec2,
};

use crate::{AppState};

pub fn mouse_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>, 
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut app_state:ResMut<AppState>
) {
    for e in mouse_button_input_events.iter() {
        if e.button == MouseButton::Left && e.state == ElementState::Pressed {
            if app_state.input_locked == false {
                app_state.input_locked = true;
            }
        }
    }

    app_state.dpad2 = Vec2::default();
    if app_state.input_locked {
        for e in mouse_motion_events.iter() {
            app_state.dpad2 += e.delta;
        }
    }
}
