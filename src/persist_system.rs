use bevy::{prelude::{World}, app::Events};

use crate::GlobalCommand;

pub fn persist_system(world:&mut World) {
    let events = world.get_resource::<Events<GlobalCommand>>().unwrap();
    for e in events.get_reader().iter(events) {
        match e {
            GlobalCommand::SaveWorld { path: _ } => {
                println!("save");
            },
            GlobalCommand::LoadWorld { path: _ } => {
                println!("load");
            },
            _ => {},
        }
    }
}