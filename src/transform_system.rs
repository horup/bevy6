use bevy::{prelude::{Query, Transform, Res}, core::Time};
use crate::PlayerInput;

pub fn player_transform_system(mut q:Query<(&mut PlayerInput, &mut Transform)>, time:Res<Time>) {
    for (player_input, mut transform) in q.iter_mut() {
        transform.translation.x += player_input.dpad.x * time.delta_seconds();
    }
}