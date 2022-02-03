use crate::{AppState};
use bevy::{
    prelude::{EventReader, ResMut, DetectChanges},
    window::Windows,
};

pub fn window_system(app_state:ResMut<AppState>, mut windows: ResMut<Windows>) {
    if let Some(primary) = windows.get_primary_mut() {
        if app_state.is_changed() {
            primary.set_cursor_visibility(!app_state.input_locked);
            primary.set_cursor_lock_mode(app_state.input_locked);
        }
    }
}
