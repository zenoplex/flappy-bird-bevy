use bevy::prelude::*;

const GRAVITY: f32 = 10.0;

#[derive(Debug)]
struct Player {
    velocity: Vec3,
}

#[derive(Debug)]
struct Pipe {}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn add_player(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let texture = asset_server.load("bird.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture.into()),
            // material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            // sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .insert(Player {
            velocity: Vec3::ZERO,
        });
}

fn add_pipes(mut commands: Commands) {
    commands.spawn_bundle((Pipe {}, Position { x: 10.0, y: 0.0 }));
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            player.velocity.y = 5.0;
        }
    }
}

fn move_system(time: Res<Time>, mut q: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut t) in q.iter_mut() {
        player.velocity.y += -GRAVITY * time.delta_seconds();
        t.translation.y += player.velocity.y;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        .add_startup_system(add_pipes.system())
        .add_system(move_system.system())
        .add_system(input_system.system())
        .run();
}
