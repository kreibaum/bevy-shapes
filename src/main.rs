use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotation, fall, remove_offscreen_enemies))
        .run();
}

const ENEMY_RADIUS: f32 = 50.0;

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>, ) {
    commands.spawn(Camera2dBundle::default());

    let pentagon = Mesh2dHandle(meshes.add(RegularPolygon::new(ENEMY_RADIUS, 5)));
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
            }, Enemy, RotationSpeed(rotation_speed), FallingSpeed(rand::random::<f32>() * 100.0 + 50.0)
            ));
        }
    }
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
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    // We query for Entity here to be able to remove it with a command.
    mut enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    for (entity, transform) in enemies.iter() {
        let mut world_position = transform.translation;
        world_position.y += ENEMY_RADIUS;
        if let Some(screen_position) = camera.world_to_viewport(camera_transform, world_position) {
            // The y amount increases as we go down the screen.
            if screen_position.y > window.height() {
                commands.entity(entity).despawn();
                println!("Enemy despawned");
            }
        }
    }
}