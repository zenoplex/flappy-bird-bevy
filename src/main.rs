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

fn add_player(mut commands: Commands) {
    commands.spawn_bundle((
        Player {},
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 0.0, y: 0.0 },
        Render {},
    ));
}

fn add_pipes(mut commands: Commands) {
    commands.spawn_bundle((Pipe {}, Position { x: 10.0, y: 0.0 }, Render {}));
}

fn hello(query: Query<&Position, With<Player>>) {
    for player in query.iter() {
        println!("{:?}", player);
    }
}

fn main() {
    App::build()
        .add_startup_system(add_player.system())
        .add_startup_system(add_pipes.system())
        .add_system(hello.system())
        .run();
}
