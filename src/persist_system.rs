use bevy::prelude::EventReader;

use crate::Command;

pub fn persist_system(mut command_reader:EventReader<Command>) {
    for e in command_reader.iter() {
        match e {
            Command::SaveWorld { path } => {
                println!("save");
            },
            Command::LoadWorld { path } => {
                println!("load");
            },
            _ => {},
        }
    }
}