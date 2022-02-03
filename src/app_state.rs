pub struct AppState {
    pub simulation_speed:f32,
    pub input_locked:bool
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            simulation_speed: 0.0,
            input_locked: false
        }
    }
}