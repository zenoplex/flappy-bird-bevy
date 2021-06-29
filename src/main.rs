use std::f32::consts::PI;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

const GRAVITY: f32 = 10.0;
const MAX_VELOCITY_Y: f32 = 200.0;
const MAX_ANGLE_UP: f32 = PI * 0.5 * 0.5;
const MAX_ANGLE_DOWN: f32 = PI * 0.5;
// Can replace with Sprite.size
const PIPE_WIDTH: f32 = 70.0;
const PIPE_HEIGHT: f32 = 430.0;

struct GameState {
    score: u32,
}

#[derive(Debug)]
struct Player {
    velocity: Vec3,
}

#[derive(Debug)]
struct Pipe;

struct Velocity(Vec2);

struct WantToFlap;

struct OffscreenDespawn;

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

struct SpawnTimer(Timer);

fn spawn_pipe(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    windows: Res<Windows>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if !spawn_timer.0.tick(time.delta()).finished() {
        return;
    }
    // TODO: calc positions
    // calc gaps
    // move pipes

    // TODO: Remove hard coded variable
    let velocity = Vec2::new(-200.0, 0.0);
    let texture = asset_server.load("pipe.png");

    if let Some(window) = windows.get_primary() {
        let pos_x = window.width() / 2.0;
        let pipe_offset_x = PIPE_WIDTH / 2.0;
        let pipe_offset_y = PIPE_HEIGHT / 2.0;
        // TODO: use rand
        let mut rng = thread_rng();
        let max_gap_size = window.height() / 4.0;
        let min_gap_size = window.height() / 10.0;
        let gap_y = rng.gen_range(0.0..(window.height() / 2.0)) - window.height() / 4.0;
        let half_gap_size = rng.gen_range(min_gap_size..max_gap_size) / 2.0;

        // Bottom
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture.clone().into()),
                transform: Transform {
                    // TODO: add random up downs
                    translation: Vec3::new(
                        pos_x + PIPE_WIDTH,
                        gap_y - pipe_offset_y - half_gap_size,
                        10.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe)
            .insert(OffscreenDespawn)
            .insert(Velocity(velocity));

        // Top
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture.into()),
                transform: Transform {
                    translation: Vec3::new(
                        pos_x + PIPE_WIDTH,
                        gap_y + pipe_offset_y + half_gap_size,
                        10.0,
                    ),
                    rotation: Quat::from_rotation_z(PI).mul_quat(Quat::from_rotation_y(PI)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe)
            .insert(OffscreenDespawn)
            .insert(Velocity(velocity));
    }
}

#[derive(Default, Clone)]
struct UiFont(Handle<Font>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Font> = asset_server.load("flappy_bird.ttf");
    commands.insert_resource(UiFont(handle));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn in_game_input_system(keyboard_input: Res<Input<KeyCode>>, mut commands: Commands) {
    if keyboard_input.pressed(KeyCode::Up) {
        commands.spawn().insert(WantToFlap {});
    }
}

fn menu_input_system(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard_input.pressed(KeyCode::Return) {
        app_state
            .set(AppState::InGame)
            .expect("Error switching app_state");
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

        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn pipe_move_system(t: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    let delta = t.delta_seconds();
    q.iter_mut().for_each(|(v, mut t)| {
        t.translation.x += v.0.x * delta;
        t.translation.y += v.0.y * delta;

        // println!("{:?}", translation.x);
    })
}

fn collistion_system(
    player_query: Query<(&Player, &Transform, &Sprite)>,
    pipe_query: Query<(&Pipe, &Transform, &Sprite)>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Ok((_, player_tranform, player_sprite)) = player_query.single() {
        pipe_query
            .iter()
            .for_each(|(_, pipe_tranform, pipe_sprite)| {
                let collision = collide(
                    player_tranform.translation,
                    player_sprite.size,
                    pipe_tranform.translation,
                    pipe_sprite.size,
                );

                if let Some(collision) = collision {
                    println!("end game, {:?}", collision);

                    app_state
                        .set(AppState::GameOver)
                        .expect("Failed to change app_state");
                }
            })
    }
}

fn offscreen_despawn_system(
    windows: Res<Windows>,
    query: Query<(Entity, &Transform)>,
    mut commands: Commands,
) {
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };

    let safe_margin = 300.0;
    let half_width = window.width() / 2.0;
    query.iter().for_each(|(entity, transform)| {
        if transform.translation.x < -half_width - safe_margin
            || transform.translation.x > half_width + safe_margin
        {
            println!("Remove entity {:?}", &entity);
            commands.entity(entity).despawn();
        }
    });
}

struct GameOverScreen;

fn game_over_system(mut commands: Commands, font: Res<UiFont>) {
    let text_style = TextStyle {
        font: font.0.clone(),
        font_size: 120.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn_bundle(Text2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 50.0),
                ..Default::default()
            },
            text: Text::with_section(
                "
Game Over
Score: 0
",
                text_style,
                text_alignment,
            ),
            ..Default::default()
        })
        .insert(GameOverScreen);
}

// should be initializing position and stuff
fn restart_game_system(
    mut commands: Commands,
    game_over_query: Query<(Entity, &GameOverScreen)>,
    pipe_query: Query<(Entity, &Pipe)>,
    mut player_query: Query<(&Player, &mut Transform)>,
    windows: Res<Windows>,
) {
    game_over_query.iter().for_each(|(entity, _)| {
        commands.entity(entity).despawn();
    });

    pipe_query.iter().for_each(|(pipe_entity, _)| {
        commands.entity(pipe_entity).despawn();
    });

    if let Some(window) = windows.get_primary() {
        player_query.iter_mut().for_each(|(_, mut transform)| {
            transform.translation = Vec3::new(-(window.width() / 10.0), 0.0, 0.0);
        });
    };
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(setup.system())
        .add_startup_system(add_player.system())
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(move_system.system())
                .with_system(in_game_input_system.system())
                .with_system(spawn_pipe.system())
                .with_system(pipe_move_system.system())
                .with_system(collistion_system.system())
                .with_system(offscreen_despawn_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::GameOver)
                .with_system(game_over_system.system())
                .with_system(menu_input_system.system())
                .with_system(offscreen_despawn_system.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::GameOver).with_system(restart_game_system.system()),
        )
        // TODO: Add menu screen
        .add_state(AppState::InGame)
        .run();
}
