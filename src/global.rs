use bevy::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Global {
    pub simulation_speed:f32,
    pub input_locked:bool,
    pub dpad:Vec2,
    pub dpad2:Vec2
}

impl Default for Global {
    fn default() -> Self {
        Self { 
            simulation_speed: 0.0,
            input_locked: false,
            dpad:Vec2::default(),
            dpad2:Vec2::default()
        }
    }
}