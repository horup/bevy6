use wasm_bindgen::prelude::*;
use bevy::{prelude::*};
mod global;
pub use global::*;
mod global_command;
pub use global_command::*;

use bevy_egui::EguiPlugin;

// systems
mod keyboard_system;
mod mouse_system;
mod window_system;
mod ui_system;
mod camera_system;
mod startup_system;
mod player_input_system;
mod transform_system;
mod persist_system;

// components
mod player_input;
pub use player_input::*;

mod first_person;
pub use first_person::*;

mod persist;
pub use persist::*;

// resources

#[wasm_bindgen]
pub fn start() {
    App::new()
    .insert_resource(WindowDescriptor {
        title:"Bevy 0.6".to_string(),
        cursor_locked:false,
        cursor_visible:true,
        ..Default::default()
    })
    .insert_resource(Global {
        ..Default::default()
    })
    .add_event::<GlobalCommand>()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)

    .add_startup_system(startup_system::startup_system)
    
    .add_system(keyboard_system::keyboard_system)
    .add_system(mouse_system::mouse_system)
    .add_system(player_input_system::player_input_system)
    .add_system(camera_system::camera_system)
    .add_system(transform_system::player_transform_system)
    
    .add_system(window_system::window_system)
    .add_system(ui_system::ui_system)
    .add_system(persist_system::persist_system.exclusive_system())
    .run();
}