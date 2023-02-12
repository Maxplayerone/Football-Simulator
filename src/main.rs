use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Component)]
pub struct PlayerMain;

#[derive(Component)]
pub struct PlayerSub;

#[derive(Component)]
pub struct Ball;

#[derive(Resource)]
pub struct GameAssets {
    goal: Handle<Scene>,
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
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(setup_scene)
        .add_system(bevy::window::close_on_esc)
        .add_system(player_sub_movement)
        .add_system(player_main_movement)
        .add_system(check_if_goal)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        goal: assets.load("goal3.glb#Scene0"),
        ball: assets.load("ball2.glb#Scene0"),
    });
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(5.0, 0.1, 5.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Grass"));

    //goal left
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(0.3, 2.0, 3.0))
        .insert(SceneBundle {
            scene: assets.goal.clone(),
            transform: Transform::from_xyz(-4.0, 1.1, 0.0).with_scale(Vec3::new(0.4, 0.4, 0.4)),
            ..Default::default()
        })
        .insert(Sensor)
        .insert(Name::new("Goal left"));

    //goal right
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(0.3, 2.0, 3.0))
        .insert(SceneBundle {
            scene: assets.goal.clone(),
            transform: Transform::from_xyz(4.0, 1.1, 0.0).with_scale(Vec3::new(0.4, 0.4, 0.4)),
            ..Default::default()
        })
        .insert(Sensor)
        .insert(Name::new("Goal right"));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.6))
        .insert(Restitution::coefficient(1.0))
        .insert(SceneBundle {
            scene: assets.ball.clone(),
            transform: Transform::from_xyz(0.0, 4.0, 0.0).with_scale(Vec3::new(0.25, 0.25, 0.25)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(Name::new("Ball"));

    //player main
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.25, 0.25))
        .insert(KinematicCharacterController::default())
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.55, 0.96, 0.96).into()),
            transform: Transform::from_xyz(-2.0, 0.25, 0.0),
            ..default()
        })
        .insert(GravityScale(0.5))
        .insert(PlayerMain)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Name::new("Player main"));

    //player sub
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.25, 0.25))
        .insert(KinematicCharacterController::default())
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.98, 0.58, 0.53).into()),
            transform: Transform::from_xyz(2.0, 0.25, 0.0),
            ..default()
        })
        .insert(GravityScale(0.5))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(PlayerSub)
        .insert(Name::new("Player Sub"));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-7.342, 7.920, 0.238)
            .with_rotation(Quat::from_xyzw(-1.579, -0.806, -1.582, 1.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

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

fn player_main_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<PlayerMain>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PlayerMain>)>,
) {
    let mut player = player_query.single_mut();
    let mut camera = camera_query.single_mut();

    let mut left = player.left();
    left.y = 0.0;
    left = left.normalize();

    let mut forward = player.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let speed = 3.0;

    if keyboard.pressed(KeyCode::A) {
        player.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        player.translation -= forward * time.delta_seconds() * speed;       
    }
    if keyboard.pressed(KeyCode::W) {
        player.translation -= left * speed * time.delta_seconds();
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        player.translation += left * speed * time.delta_seconds();
        camera.translation += left * time.delta_seconds() * speed;
    }
}

fn player_sub_movement(
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

    if keyboard.pressed(KeyCode::Left) {
        player.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Right) {
        player.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Up) {
        player.translation -= left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        player.translation += left * speed * time.delta_seconds();
    }
}

fn check_if_goal(ball_transform: Query<&Transform, With<Ball>>) {
    let ball = ball_transform.single();

    if ball.translation.x > 4.0 || ball.translation.x < -4.0 {
        println!("gal");
    }
}