use std::f32::consts::PI;

use bevy::prelude::*;

const GRAVITY: f32 = 10.0;
const MAX_VELOCITY_Y: f32 = 200.0;
const MAX_ANGLE_UP: f32 = PI * 0.5 * 0.5;
const MAX_ANGLE_DOWN: f32 = PI * 0.5;

#[derive(Debug)]
struct Player {
    velocity: Vec3,
}

#[derive(Debug)]
struct Pipe {}

struct WantToFlap {}

fn add_player(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    windows: Res<Windows>,
) {
    let texture = asset_server.load("bird.png");
    if let Some(window) = windows.get_primary() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture.into()),
                transform: Transform {
                    translation: Vec3::new(-(window.width() / 10.0), 0.0, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player {
                velocity: Vec3::ZERO,
            });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn input_system(keyboard_input: Res<Input<KeyCode>>, mut commands: Commands) {
    if keyboard_input.pressed(KeyCode::Up) {
        commands.spawn().insert(WantToFlap {});
    }
}

fn move_system(
    time: Res<Time>,
    // Maybe use QuerySet?
    mut q: Query<(&mut Player, &mut Transform)>,
    mut q2: Query<(Entity, &WantToFlap)>,
    mut commands: Commands,
) {
    for (mut player, mut transform) in q.iter_mut() {
        let delta = time.delta_seconds();
        if let Ok((entity, _)) = q2.single_mut() {
            player.velocity.y = 5.0;
            commands.entity(entity).despawn();
        }

        player.velocity.y += -GRAVITY * delta;
        // Clamp terminal velocity
        player.velocity.y = player.velocity.y.max(-MAX_VELOCITY_Y);
        transform.translation.y += player.velocity.y;
        let angle = player
            .velocity
            .y
            .atan2(1.0)
            .clamp(-MAX_ANGLE_DOWN, MAX_ANGLE_UP);

        println!("{}", angle);

        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        .add_system(move_system.system())
        .add_system(input_system.system())
        .run();
}
