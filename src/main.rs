use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

// Game Coordinate Space:
// X: -300 (left) to 300 (right)
// Y: 0 (bottom) to 800 (top)

const ENEMY_RADIUS: f32 = 50.0;
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Shape Shooter".to_string(),
                    resolution: (600., 800.).into(),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_systems(Startup, (setup_camera, setup_enemy_spawner))
        .add_systems(Update, (rotation, fall, remove_offscreen_enemies, spawn_enemy))
        .run();
}


fn setup_camera(mut commands: Commands,) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 400.0, 1.0)),
        ..default()
    });
}

#[derive(Component)]
struct RotationSpeed(f32);

#[derive(Component)]
struct FallingSpeed(f32);

#[derive(Component)]
struct Enemy;

fn rotation(time: Res<Time>, mut query: Query<(&RotationSpeed, &mut Transform)>) {
    for (rotation, mut transform) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * rotation.0);
    }
}

fn fall(time: Res<Time>, mut query: Query<(&FallingSpeed, &mut Transform)>) {
    for (falling, mut transform) in query.iter_mut() {
        transform.translation.y -= falling.0 * time.delta_seconds();
    }
}

fn remove_offscreen_enemies(
    mut commands: Commands,
    // We query for Entity here to be able to remove it with a command.
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in enemies.iter() {
        let world_position = transform.translation;
        if world_position.y + ENEMY_RADIUS < 0. {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct EnemySpawner {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

fn setup_enemy_spawner(mut commands: Commands,
                       mut meshes: ResMut<Assets<Mesh>>,
                       mut materials: ResMut<Assets<ColorMaterial>>) {
    let pentagon = Mesh2dHandle(meshes.add(RegularPolygon::new(ENEMY_RADIUS, 5)));
    let red = Color::hsl(0.0, 0.5, 0.5);

    commands.spawn(EnemySpawner {
        mesh: pentagon,
        material: materials.add(red),
    });
}

// There is a 10% chane of spawning an enemy every second
fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    spawner: Query<&EnemySpawner>,
) {
    let spawner = spawner.single();

    if rand::random::<f32>() < time.delta_seconds() {
        let rotation_speed = if rand::random::<bool>() {
            rand::random::<f32>().powi(2) + 0.1
        } else {
            -rand::random::<f32>().powi(2) - 0.1
        };

        let start_position = Vec3::new(- WINDOW_WIDTH / 2. + ENEMY_RADIUS + rand::random::<f32>() * (WINDOW_WIDTH - 2.0 * ENEMY_RADIUS), WINDOW_HEIGHT, 0.);
        commands.spawn((MaterialMesh2dBundle {
            mesh: spawner.mesh.clone(),
            material: spawner.material.clone(),
            transform: Transform::from_translation(start_position),
            ..Default::default()
        }, Enemy, RotationSpeed(rotation_speed), FallingSpeed(rand::random::<f32>() * 100.0 + 50.0)
        ));
    }
}