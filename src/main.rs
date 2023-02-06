use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Component)]
pub struct PlayerMain;

#[derive(Component)]
pub struct PlayerSub;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
            width: WIDTH,
            height: HEIGHT,
            title: "Football Simulator".to_string(),
            ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(draw_field)
        .add_startup_system(camera_spawn)
        .add_system(bevy::window::close_on_esc)
        .add_system(camera_controls)
        .add_system(player_movement)
        .add_system(second_player_movement)
        .run();
}

fn draw_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    //field
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane{
            size: 10.0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Grass"));

    //player
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{
            size: 0.5,
        })),
        material: materials.add(Color::rgb(0.55, 0.96, 0.96).into()),
        transform: Transform::from_xyz(-2.0, 0.25, 0.0),
        ..default()
    })
    .insert(PlayerMain)
    .insert(Name::new("Player main"));

    //player
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{
            size: 0.5,
        })),
        material: materials.add(Color::rgb(0.98, 0.58, 0.53).into()),
        transform: Transform::from_xyz(2.0, 0.25, 0.0),
        ..default()
    })
    .insert(PlayerSub)
    .insert(Name::new("Player Sub"));

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
){
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
    if keyboard.pressed(KeyCode::D){
        player.translation -= left * speed * time.delta_seconds(); 
    }
    if keyboard.pressed(KeyCode::A){
        player.translation += left * speed * time.delta_seconds(); 
    }
}

fn second_player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<PlayerSub>>,
){
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
    if keyboard.pressed(KeyCode::Right){
        player.translation -= left * speed * time.delta_seconds(); 
    }
    if keyboard.pressed(KeyCode::Left){
        player.translation += left * speed * time.delta_seconds(); 
    }
}

fn camera_spawn(mut commands: Commands){
    commands
    .spawn(Camera3dBundle {
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
