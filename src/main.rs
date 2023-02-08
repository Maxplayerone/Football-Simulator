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
*/

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

mod camera;
use camera::CustomCameraPlugin;

#[derive(Component)]
pub struct Player;

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
        .add_plugin(CustomCameraPlugin)
        .add_startup_system(setup_physics)
        .add_system(player_movement)
        .run();
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
        .insert(Restitution::coefficient(0.4))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

        
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(KinematicCharacterController::default())
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.98, 0.58, 0.53).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Player); 
        /*
        commands.spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
        */
}

//TODO: PLAYER MOVEMENT

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player = player_query.single_mut();   
    let mut left = player.left();
    left.y = 0.0;
    left = left.normalize();

    let mut forward = player.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let speed = 5.0;

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
