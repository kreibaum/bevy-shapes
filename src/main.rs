use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

// use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, enemy_movement)
//        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>, ) {
    commands.spawn(Camera2dBundle::default());

    let pentagon = Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 5)));
    let red = Color::hsl(0.0, 0.5, 0.5);

    // Spawn a 5x5 enemy grid

    for x in -2..=2 {
        for y in -2..=2 {
            let rotation_speed = if rand::random::<bool>() {
                rand::random::<f32>().powi(2) + 0.1
            } else {
                -rand::random::<f32>().powi(2) - 0.1
            };

            commands.spawn((MaterialMesh2dBundle {
                mesh: pentagon.clone(),
                material: materials.add(red),
                transform: Transform::from_translation(Vec3::new(x as f32 * 100.0, y as f32 * 100.0, 0.0)),
                ..Default::default()
            }, Enemy, RotationSpeed(rotation_speed)
            ));
        }
    }
}

#[derive(Component)]
struct RotationSpeed(f32);

#[derive(Component)]
struct Enemy;

fn enemy_movement(time: Res<Time>, mut query: Query<(&RotationSpeed, &mut Transform)>) {
    for (rotation, mut transform) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * rotation.0);
    }
}