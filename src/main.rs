/*
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use std::f32::consts::PI;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Component)]
pub struct PlayerMain;

#[derive(Component)]
pub struct PlayerSub;

#[derive(Resource)]
pub struct GameAssets {
    ball: Handle<Scene>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Football Simulator".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(draw_field)
        .add_startup_system(camera_spawn)
        .add_system(bevy::window::close_on_esc)
        .add_system(camera_controls)
        .add_system(player_movement)
        .add_system(second_player_movement)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        ball: assets.load("ball.glb#Scene0"),
    });
}

fn draw_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    //field
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Grass"));

    //player main
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.55, 0.96, 0.96).into()),
            transform: Transform::from_xyz(-2.0, 0.25, 0.0),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(KinematicCharacterController::default())
        .insert(PlayerMain)
        .insert(Name::new("Player main"));

    //player sub
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.98, 0.58, 0.53).into()),
            transform: Transform::from_xyz(2.0, 0.25, 0.0),
            ..default()
        })
        .insert(PlayerSub)
        .insert(Name::new("Player Sub"));
    //ball
    /*
        commands
            .spawn(SceneBundle {
                scene: game_assets.ball.clone(),
                transform:  Transform::from_xyz(0.0, 0.7, 0.6)
                    .with_rotation(Quat::from_rotation_y(-PI / 2.0))
                    .with_scale(Vec3::new(0.05, 0.05, 0.05)),
                ..Default::default()
            })
            .insert(Name::new("Ball"));
    */
    //light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<PlayerMain>>,
) {
    let mut player = player_query.single_mut();

    let mut left = player.left();
    left.y = 0.0;
    left = left.normalize();

    let mut forward = player.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let speed = 3.0;

    if keyboard.pressed(KeyCode::W) {
        player.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        player.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        player.translation -= left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        player.translation += left * speed * time.delta_seconds();
    }
}

fn second_player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<PlayerSub>>,
) {
    let mut player = player_query.single_mut();

    let mut left = player.left();
    left.y = 0.0;
    left = left.normalize();

    let mut forward = player.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let speed = 3.0;

    if keyboard.pressed(KeyCode::Up) {
        player.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Down) {
        player.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Right) {
        player.translation -= left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        player.translation += left * speed * time.delta_seconds();
    }
}

fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.037, 2.5, 8.372).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 1.5;

    if keyboard.pressed(KeyCode::I) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::K) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::J) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::L) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::U) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::O) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}
*/

use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Resource, Default)]
struct EulerAngles {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Football Simulator".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .init_resource::<EulerAngles>()
        .add_startup_system(setup_physics)
        .add_startup_system(camera_spawn)
        .add_system(camera_controls)
        .add_system(player_look)
        .add_system(cursor_grab)
        .add_startup_system(initial_grab_cursor)
        //.add_startup_system(initial_grab_cursor)
        .run();
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Key1) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(10.0, 0.1, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("plane"));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.037, 2.5, 8.372).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    //we need to zero the y value because the camera look-at is tilting
    //so it fucks up with the forward/backward movement
    forward = forward.normalize();

    let mut left = camera.left();
    left = left.normalize();

    let speed = 3.0;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::LControl) {
        camera.translation -= Vec3::Y * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Space) {
        camera.translation += Vec3::Y * time.delta_seconds() * speed;
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    mut euler_angles: ResMut<EulerAngles>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = euler_angles.as_mut();
        let mut camera_transform = query.single_mut();

        let window_scale = 720.0;
        let sens: f32 = 0.00012;
        for ev in delta_state.reader_motion.iter(&motion) {
            match window.cursor_grab_mode() {
                CursorGrabMode::None => (),
                _ => {
                    let window_scale = 720.0;
                    delta_state.pitch -=
                        (sens * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (sens * ev.delta.x * window_scale).to_radians();
                }
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
        }

        // Order is important to prevent unintended roll
        camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
            * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
