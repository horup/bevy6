use std::f32::consts::PI;

use bevy::{prelude::{Query, Transform, Res}, core::Time, math::{Quat, Vec3}};
use crate::{PlayerInput, FirstPerson};

pub fn player_transform_system(mut q:Query<(&mut PlayerInput, &mut Transform, &mut FirstPerson)>, time:Res<Time>) {
    let dt = time.delta_seconds();
    for (player_input, mut transform, mut first_person) in q.iter_mut() {
        let move_speed = 10.0;
        let rot_speed = 0.1;

        first_person.yaw += -player_input.dpad2.x * rot_speed * dt;
        first_person.pitch += -player_input.dpad2.y * rot_speed * dt;
        first_person.pitch = first_person.pitch.clamp(-PI/2.1, PI/2.1);

        let forward = Quat::from_rotation_y(first_person.yaw) * Vec3::new(0.0, 0.0, -1.0);
        let right = Quat::from_rotation_y(first_person.yaw) * Vec3::new(1.0, 0.0, 0.0);
        let right = right.normalize();

        let forward = Quat::from_axis_angle(right, first_person.pitch) * forward;
        let target = transform.translation + forward;

        transform.look_at(target, Vec3::Y);
    

        transform.translation += forward * player_input.dpad.y * dt * move_speed;
        transform.translation += right * player_input.dpad.x * dt * move_speed;
    }
}