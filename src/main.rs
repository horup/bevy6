use bevy::{prelude::*, input::mouse::MouseButtonInput};

#[derive(Component, Debug)]
struct Player {

}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });


    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(Player {

    });
}
type PlayerEntity = (&'static mut Transform, &'static mut Camera);

fn mouse_input_system(mut q: Query<PlayerEntity>) {

}

fn keyboard_input_system(mut q:Query<PlayerEntity>, keyboard_input: Res<Input<KeyCode>>, time:Res<Time>) {
    let q = q.get_single_mut();
    if let Ok((mut transform, _camera)) = q {
        let mut v = Vec3::new(0.0, 0.0, 0.0);
        if keyboard_input.pressed(KeyCode::A) {
            v.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            v.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            v.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            v.z += 1.0;
        }

        let mut v = transform.rotation * v;
        v.y = 0.0;
        let speed = 10.0;
        transform.translation += v * time.delta_seconds() * speed;
    }
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(keyboard_input_system)
    .add_system(mouse_input_system)
    .run();
}