use std::f32::consts::PI;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

const FLAP_VELOCITY_Y: f32 = 300.0;
const GRAVITY: f32 = 1000.0;
const PIPE_VELOCTY_X: f32 = 250.0;
const MAX_VELOCITY_Y: f32 = 500.0;
const MAX_ANGLE_DOWN: f32 = -PI * 0.5;
const MAX_ANGLE_UP: f32 = PI * 0.3;
const BASE_HEIGHT: f32 = 112.0;
const PIPE_WIDTH: f32 = 70.0;
const PIPE_HEIGHT: f32 = 500.0;

struct GameState {
    score: u32,
}

#[derive(Debug)]
struct Player;

struct Gravity(Vec2);

#[derive(Debug)]
struct Pipe;

struct Velocity(Vec2);

struct WantToFlap;

struct OffscreenDespawn;

struct Parallax {
    // Maybe use Vecolity
    velocity_x: f32,
    loop_x: f32,
}

struct SpawnTimer(Timer);

fn pipe_system(
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
    let velocity = Vec2::new(-PIPE_VELOCTY_X, 0.0);
    let texture = asset_server.load("pipe.png");

    if let Some(window) = windows.get_primary() {
        let available_height = (window.height() / 2.0) - BASE_HEIGHT;
        let pos_x = window.width() / 2.0;
        let pipe_offset_y = PIPE_HEIGHT / 2.0;
        let mut rng = thread_rng();
        // Gap size should be based from player size
        let max_gap_size = available_height;
        let min_gap_size = available_height / 3.5;
        let gap_y = rng.gen_range(0.0..available_height / 2.0);
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        let texture = asset_server.load("bird.png");
        if let Some(window) = windows.get_primary() {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(texture.into()),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle((
                    Player,
                    Velocity(Vec2::ZERO),
                    Gravity(Vec2::new(0.0, GRAVITY)),
                ));
        }

        let handle: Handle<Font> = asset_server.load("flappy_bird.ttf");
        commands.insert_resource(UiFont(handle));
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        let background_texture = asset_server.load("background.png");
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(background_texture.into()),
                // Using Sprite to increase size instead of scale because we rely on Sprite.size
                sprite: Sprite::new(Vec2::new(2760.0, 720.0)),
                transform: Transform {
                    translation: Vec3::new(690.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Parallax {
                velocity_x: 80.0,
                loop_x: 1380.0,
            });

        let base_texture = asset_server.load("base.png");
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(base_texture.into()),
                transform: Transform {
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    translation: Vec3::new(
                        648.0,
                        -window.height() / 2.0 + (BASE_HEIGHT / 2.0),
                        200.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Parallax {
                // Should be same as pipe speed
                velocity_x: PIPE_VELOCTY_X,
                loop_x: 1296.0,
            });
    }
}

fn parallax_system(time: Res<Time>, mut query: Query<(&Parallax, &mut Transform)>) {
    query.iter_mut().for_each(|(parallax, mut transform)| {
        let offset_x = parallax.loop_x / 2.0;
        // Looping from -loop_x to loop_x
        transform.translation.x = -((-(transform.translation.x - offset_x)
            + parallax.velocity_x * time.delta_seconds())
            % parallax.loop_x)
            + offset_x;
    });
}

fn in_game_input_system(keyboard_input: Res<Input<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn().insert(WantToFlap {});
    }
}

fn menu_input_system(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        app_state
            .set(AppState::InGame)
            .expect("Error switching app_state");
    }
}

fn flap_system(
    time: Res<Time>,
    // Maybe use QuerySet?
    mut query_player: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut query_intent: Query<(Entity, &WantToFlap)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (mut transform, mut velocity) in query_player.iter_mut() {
        let delta = time.delta_seconds();

        if let Ok((entity, _)) = query_intent.single_mut() {
            println!("flap");
            velocity.0.y = FLAP_VELOCITY_Y;

            let sound = asset_server.load("audio_wing.wav");
            audio.play(sound);

            commands.entity(entity).despawn();
        }

        // Should use lerp
        let angle = (velocity.0.y * delta)
            .atan2(1.0)
            .clamp(MAX_ANGLE_DOWN, MAX_ANGLE_UP);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn move_system(
    t: Res<Time>,
    mut q: Query<(&mut Velocity, &mut Transform, Option<&Gravity>)>,
    app_state: Res<State<AppState>>,
) {
    let delta = t.delta_seconds();
    q.iter_mut().for_each(|(mut v, mut t, gravity)| {
        t.translation.x += v.0.x * delta;
        t.translation.y += v.0.y * delta;

        if let Some(gravity) = gravity {
            // Maybe remove Gravity component from player
            if *app_state.current() != AppState::InGame {
                return;
            }

            v.0.y += (-gravity.0.y * delta).max(-MAX_VELOCITY_Y);
            // Clamp terminal velocity
            v.0.y = v.0.y.max(-MAX_VELOCITY_Y);
        }
    })
}

fn collistion_system(
    player_query: Query<(&Player, &Transform, &Sprite)>,
    pipe_query: Query<(&Pipe, &Transform, &Sprite)>,
    mut app_state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((_, player_tranform, player_sprite)) = player_query.single() {
        pipe_query
            .iter()
            .for_each(|(_, pipe_tranform, pipe_sprite)| {
                let collision = collide(
                    player_tranform.translation,
                    // Slightly make hitbox smaller
                    player_sprite.size * 0.8,
                    pipe_tranform.translation,
                    pipe_sprite.size,
                );

                if let Some(collision) = collision {
                    println!("end game, {:?}", collision);

                    let sound = asset_server.load("audio_hit.wav");
                    audio.play(sound);

                    app_state
                        .set(AppState::GameOver)
                        .expect("Failed to change app_state");
                }
            })
    }
}

fn boundary_system(
    windows: Res<Windows>,
    mut app_state: ResMut<State<AppState>>,
    mut player_query: Query<(&mut Transform, &Sprite, &mut Velocity), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };

    let half_height = window.height() / 2.0;

    player_query
        .iter_mut()
        .for_each(|(mut transform, sprite, mut velocity)| {
            let player_half_height = sprite.size.y / 2.0;

            if transform.translation.y <= -(half_height - player_half_height - BASE_HEIGHT) {
                let sound = asset_server.load("audio_hit.wav");
                audio.play(sound);

                app_state
                    .set(AppState::GameOver)
                    .expect("Failed to change state");
            };

            // Reflect if player hit the ceiling
            if transform.translation.y > (half_height - player_half_height) {
                velocity.0 *= -1.0;
                transform.translation.y = half_height - player_half_height;
            };
        });
}

fn offscreen_despawn_system(
    windows: Res<Windows>,
    query: Query<(Entity, &Transform), With<Pipe>>,
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

struct MainMenuScreen;

fn main_menu_system(mut commands: Commands, font: Res<UiFont>) {
    // Consider using image because text is slow
    let text_style = TextStyle {
        font: font.0.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn_bundle(Text2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -60.0, 50.0),
                ..Default::default()
            },
            text: Text::with_section("Press Space to play", text_style, text_alignment),
            ..Default::default()
        })
        .insert(MainMenuScreen);
}

struct GameOverScreen;

fn game_over_system(mut commands: Commands, font: Res<UiFont>) {
    // Consider using image because text is slow
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
            text: Text::with_section("Game Over", text_style, text_alignment),
            ..Default::default()
        })
        .insert(GameOverScreen);
}

// should be initializing position and stuff
fn restart_game_system(
    mut commands: Commands,
    game_over_query: Query<(Entity, Option<&GameOverScreen>, Option<&MainMenuScreen>)>,
    pipe_query: Query<(Entity, &Pipe)>,
    mut player_query: Query<(&Player, &mut Transform, &mut Velocity)>,
) {
    game_over_query
        .iter()
        .for_each(|(entity, game_over_screen, main_menu_screen)| {
            if game_over_screen.is_some() {
                commands.entity(entity).despawn();
            }

            if main_menu_screen.is_some() {
                commands.entity(entity).despawn();
            }
        });

    pipe_query.iter().for_each(|(pipe_entity, _)| {
        commands.entity(pipe_entity).despawn();
    });

    player_query
        .iter_mut()
        .for_each(|(_, mut transform, mut velocity)| {
            transform.translation = Vec3::new(0.0, 0.0, 100.0);
            velocity.0 = Vec2::ZERO
        });
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
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(main_menu_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(move_system.system())
                .with_system(menu_input_system.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(restart_game_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(flap_system.system())
                .with_system(parallax_system.system())
                .with_system(in_game_input_system.system())
                .with_system(pipe_system.system())
                .with_system(move_system.system())
                .with_system(collistion_system.system())
                .with_system(boundary_system.system())
                .with_system(offscreen_despawn_system.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver).with_system(game_over_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::GameOver)
                .with_system(menu_input_system.system())
                .with_system(offscreen_despawn_system.system()),
        )
        .add_state(AppState::MainMenu)
        .run();
}
