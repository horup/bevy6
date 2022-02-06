use bevy::prelude::{Res, Transform, Query};
use crate::{Global, PlayerInput};

pub fn player_input_system(app_state:Res<Global>, mut player_inputs:Query<&mut PlayerInput>) {
    for mut input in player_inputs.iter_mut() {
        input.dpad = app_state.dpad.clone();
        input.dpad2 = app_state.dpad2.clone();
    }
}