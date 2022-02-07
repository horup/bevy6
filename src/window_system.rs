use crate::{Global};
use bevy::{
    prelude::{EventReader, ResMut, DetectChanges},
    window::Windows, math::Vec2,
};

pub fn window_system(app_state:ResMut<Global>, mut windows: ResMut<Windows>) {
    if let Some(primary) = windows.get_primary_mut() {
        if app_state.is_changed() {
            primary.set_cursor_visibility(!app_state.input_locked);
            primary.set_cursor_lock_mode(app_state.input_locked);

            if app_state.input_locked {
                primary.set_cursor_position(Vec2::new(primary.width() / 2.0, primary.height()/ 2.0));
            }
        }
    }
}
