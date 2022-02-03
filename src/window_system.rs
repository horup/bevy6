use crate::AppEvent;
use bevy::{
    prelude::{EventReader, ResMut},
    window::Windows,
};

pub fn window_system(mut app_event_reader: EventReader<AppEvent>, mut windows: ResMut<Windows>) {
    if let Some(primary) = windows.get_primary_mut() {
        for e in app_event_reader.iter() {
            match e {
                AppEvent::LockInput(b) => {
                    primary.set_cursor_lock_mode(*b);
                    primary.set_cursor_visibility(!b);
                },
                _=>{}
            }
        }
    }
}
