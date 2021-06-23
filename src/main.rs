use bevy::prelude::*;

#[derive(Debug)]
struct Player {}

#[derive(Debug)]
struct Pipe {}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Velocity {
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
        .insert_bundle((Player {}, Velocity { x: 0.0, y: 1.0 }));
}

fn add_pipes(mut commands: Commands) {
    commands.spawn_bundle((Pipe {}, Position { x: 10.0, y: 0.0 }));
}

struct GameTimer(Timer);

fn hello(time: Res<Time>, mut timer: ResMut<GameTimer>, query: Query<&Position, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for player in query.iter() {
            println!("{:?}", player);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    if let Ok((_, mut transform)) = query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction -= 1.0;
        }

        let translation = &mut transform.translation;
        translation.y += direction;
        translation.y = translation.y.min(100.0).max(-200.0);
    }
}

fn move_system(time: Res<Time>, mut q: Query<(&Player, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (v, mut t) in q.iter_mut() {
        t.translation.y += delta * 10.0;
        // t.rotate(Quat::from_rotation_z(v.rotation * delta));
        println!("{:?}", t.translation.y);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameTimer(Timer::from_seconds(5.0, true)))
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        // .add_startup_system(add_pipes.system())
        .add_system(hello.system())
        .add_system(move_system.system())
        .add_system(input_system.system())
        .run();
}
