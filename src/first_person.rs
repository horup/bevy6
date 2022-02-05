use bevy::{math::Vec3, prelude::Component};

#[derive(Component, Clone, Copy)]
pub struct FirstPerson {
    pub yaw:f32,
    pub pitch:f32
}

impl Default for FirstPerson {
    fn default() -> Self {
        Self {
            yaw:0.0,
            pitch:0.0
        }
    }
}
