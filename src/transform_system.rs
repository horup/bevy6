use bevy::{prelude::{Query, Transform, Res}, core::Time, math::{Quat, Vec3}};
use crate::PlayerInput;

pub fn player_transform_system(mut q:Query<(&mut PlayerInput, &mut Transform)>, time:Res<Time>) {
    let dt = time.delta_seconds();
    for (player_input, mut transform) in q.iter_mut() {
        let move_speed = 10.0;
        let rot_speed = 0.1;
        
        
        let right = transform.rotation * Vec3::new(1.0, 0.0, 0.0);
        transform.rotate(Quat::from_axis_angle(right, -player_input.dpad2.y * rot_speed * dt));
        let up = transform.rotation * Vec3::new(0.0, 1.0, 0.0);
        transform.rotate(Quat::from_axis_angle(up, -player_input.dpad2.x * rot_speed * dt));

        let v = transform.rotation * Vec3::new(player_input.dpad.x, 0.0, -player_input.dpad.y).normalize_or_zero();
        transform.translation += v * move_speed * dt;


    }
}