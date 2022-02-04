use bevy::{prelude::Component, math::Vec2};

#[derive(Component, Default)]
pub struct PlayerInput {
    pub dpad:Vec2,
    pub dpad2:Vec2
}