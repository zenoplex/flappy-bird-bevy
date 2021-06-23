use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const GRAVITY: f32 = 5.0;

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

struct GameTimer(Timer);

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

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            player.velocity.y = 5.0;
        }

        // let translation = &mut transform.translation;
        // translation.y += direction;
        // translation.y = translation.y.min(100.0).max(-200.0);
    }
}

fn move_system(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut q: Query<(&mut Player, &mut Transform)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    };

    // if !timer.0.finished() {
    //     println!("not yet");
    //     return;
    // }

    for (mut player, mut t) in q.iter_mut() {
        player.velocity.y -= GRAVITY * time.delta_seconds();
        t.translation.y += player.velocity.y;
    }
}

fn game_timer_system(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.0.tick(time.delta());
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Maybe try fixed time step?
        .insert_resource(GameTimer(Timer::from_seconds(TIME_STEP, true)))
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        .add_startup_system(add_pipes.system())
        .add_system(hello.system())
        .add_system(move_system.system())
        .add_system(input_system.system())
        .run();
}
