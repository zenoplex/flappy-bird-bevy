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
struct Render {}

fn add_player(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let texture = asset_server.load("bird.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture.into()),
            ..Default::default()
        })
        .insert_bundle((Player {}, Velocity { x: 0.0, y: 0.0 }, Render {}));
}

fn add_pipes(mut commands: Commands) {
    commands.spawn_bundle((Pipe {}, Position { x: 10.0, y: 0.0 }, Render {}));
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

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameTimer(Timer::from_seconds(5.0, true)))
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        .add_startup_system(add_pipes.system())
        .add_system(hello.system())
        .run();
}
