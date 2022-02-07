use bevy::{prelude::{EventReader, World}, app::Events};

use crate::GlobalCommand;

pub fn persist_system(world:&mut World) {
    let events = world.get_resource::<Events<GlobalCommand>>().unwrap();
    for e in events.get_reader().iter(events) {
        match e {
            GlobalCommand::SaveWorld { path } => {
                println!("save");
            },
            GlobalCommand::LoadWorld { path } => {
                println!("load");
            },
            _ => {},
        }
    }
}