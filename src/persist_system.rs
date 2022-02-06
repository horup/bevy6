use bevy::prelude::EventReader;

use crate::GlobalCommand;

pub fn persist_system(mut command_reader:EventReader<GlobalCommand>) {
    for e in command_reader.iter() {
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